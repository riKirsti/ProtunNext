//! Bounded connection capture. Only the writer thread touches the sink.
use super::{
    capture::{CaptureController, CaptureInput},
    types::{CaptureStopReason, Event, PcapFileInfo},
};
use crate::{Error, Result};
use std::{
    collections::VecDeque,
    io,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant, UNIX_EPOCH},
};
use zeroize::Zeroizing;

const RECORDS: usize = 64;
const CONTROL_TIMEOUT: Duration = Duration::from_secs(1);

pub(super) enum Request {
    Start(CaptureInput),
    Stop { disconnected: bool },
}

pub(super) struct Pending {
    reply: mpsc::Receiver<Result<()>>,
    deadline: Instant,
}

enum Work {
    Record(Duration, Zeroizing<Vec<u8>>),
    Control(Request, mpsc::SyncSender<Result<()>>),
    Overloaded,
}

#[derive(Default)]
struct Queue {
    work: VecDeque<Work>,
    pending: bool,
    expired: bool,
    closed: bool,
}

#[derive(Default)]
struct Writer {
    queue: Mutex<Queue>,
    ready: Condvar,
    active: AtomicBool,
    cancel: Arc<AtomicBool>,
    events: Mutex<VecDeque<Event>>,
}

/// Retained across peer and socket replacement.
#[derive(Default)]
pub(crate) struct CaptureState {
    writer: Arc<Writer>,
    thread: Mutex<Option<JoinHandle<()>>>,
}

impl CaptureState {
    pub(crate) fn active(&self) -> bool {
        self.writer.active.load(Ordering::Relaxed)
    }

    pub(crate) fn record(&self, bytes: &[u8]) {
        if !self.active() || bytes.len() > 65535 {
            return;
        }
        let mut queue = self.writer.queue.lock().unwrap_or_else(|e| e.into_inner());
        if !self.active() || queue.closed {
            return;
        }
        if queue.work.len() >= RECORDS {
            self.writer.active.store(false, Ordering::Relaxed);
            self.writer.cancel.store(true, Ordering::Relaxed);
            queue.work.push_back(Work::Overloaded);
        } else {
            let wall = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            queue
                .work
                .push_back(Work::Record(wall, Zeroizing::new(bytes.to_vec())));
        }
        self.writer.ready.notify_one();
    }

    pub(crate) fn start(&self, info: PcapFileInfo) -> Result<()> {
        self.submit(Request::Start(CaptureInput::take(info)?), false)?
            .map_or(Ok(()), |pending| self.wait(pending, CONTROL_TIMEOUT))
    }

    pub(super) fn submit(&self, request: Request, buffer_initial: bool) -> Result<Option<Pending>> {
        {
            let mut thread = self.thread.lock().unwrap_or_else(|e| e.into_inner());
            if thread.is_none() {
                if let Request::Stop { disconnected } = request {
                    if !disconnected {
                        self.writer.event(Event::PacketCaptureStopped {
                            reason: CaptureStopReason::AlreadyStopped,
                        });
                    }
                    return Ok(None);
                }
                let writer = self.writer.clone();
                *thread = Some(
                    thread::Builder::new()
                        .name("protun-next-capture".into())
                        .spawn(move || writer.run())?,
                );
            }
        }
        self.writer.submit(request, buffer_initial).map(Some)
    }

    pub(crate) fn stop(&self, disconnected: bool) -> Result<()> {
        self.stop_with_timeout(disconnected, CONTROL_TIMEOUT)
    }

    pub(super) fn stop_with_timeout(&self, disconnected: bool, timeout: Duration) -> Result<()> {
        self.submit(Request::Stop { disconnected }, false)?
            .map_or(Ok(()), |pending| self.wait(pending, timeout))
    }

    pub(super) fn wait(&self, pending: Pending, timeout: Duration) -> Result<()> {
        pending
            .reply
            .recv_timeout(timeout)
            .unwrap_or_else(|_| self.writer.expire(&pending.reply))
    }

    pub(super) fn poll(&self, pending: &Pending) -> Option<Result<()>> {
        match pending.reply.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Empty) if Instant::now() < pending.deadline => None,
            Err(_) => Some(self.writer.expire(&pending.reply)),
        }
    }

    pub(super) fn busy(&self) -> bool {
        self.writer
            .queue
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pending
    }

    pub(crate) fn take_events(&self) -> Vec<Event> {
        self.writer
            .events
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .drain(..)
            .collect()
    }
}

impl Drop for CaptureState {
    fn drop(&mut self) {
        self.writer.close();
    }
}

impl Writer {
    fn event(&self, event: Event) {
        let mut events = self.events.lock().unwrap_or_else(|e| e.into_inner());
        if events.len() == 128 {
            events.pop_front();
        }
        events.push_back(event);
    }

    fn close(&self) {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        self.active.store(false, Ordering::Relaxed);
        self.cancel.store(true, Ordering::Relaxed);
        queue.closed = true;
        self.ready.notify_one();
    }

    fn submit(&self, request: Request, buffer_initial: bool) -> Result<Pending> {
        let shutdown = matches!(request, Request::Stop { disconnected: true });
        let (response, received) = mpsc::sync_channel(1);
        {
            let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
            if queue.closed || queue.pending {
                drop(queue);
                if shutdown {
                    self.close();
                }
                return Err(Error::Invalid("capture writer busy"));
            }
            if matches!(request, Request::Stop { .. }) {
                self.active.store(false, Ordering::Relaxed);
            } else if buffer_initial {
                // Queue the earliest packets while the initial header is prepared.
                self.active.store(true, Ordering::Relaxed);
            }
            queue.pending = true;
            queue.work.push_back(Work::Control(request, response));
            self.ready.notify_one();
        }
        Ok(Pending {
            reply: received,
            deadline: Instant::now() + CONTROL_TIMEOUT,
        })
    }

    fn expire(&self, reply: &mpsc::Receiver<Result<()>>) -> Result<()> {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        if let Ok(result) = reply.try_recv() {
            return result;
        }
        queue.expired = true;
        self.active.store(false, Ordering::Relaxed);
        self.cancel.store(true, Ordering::Relaxed);
        Err(io::Error::new(io::ErrorKind::TimedOut, "capture writer did not respond").into())
    }

    fn run(&self) {
        let mut capture = CaptureController::new();
        loop {
            let work = {
                let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                while queue.work.is_empty() && !queue.closed {
                    queue = self.ready.wait(queue).unwrap_or_else(|e| e.into_inner());
                }
                if queue.closed {
                    queue.work.clear();
                    break;
                }
                queue.work.pop_front().expect("capture work available")
            };
            match work {
                Work::Record(wall, bytes) => {
                    if self.cancel.load(Ordering::Relaxed) {
                        continue;
                    }
                    match capture.packet(wall, &bytes) {
                        Ok(Some(event)) => {
                            self.active.store(false, Ordering::Relaxed);
                            self.event(event);
                        }
                        Ok(None) => {}
                        Err(error) if !self.cancel.load(Ordering::Relaxed) => {
                            self.active.store(false, Ordering::Relaxed);
                            log::error!("capture write failed: {error}");
                            capture = CaptureController::new();
                        }
                        Err(_) => {}
                    }
                }
                Work::Overloaded => {
                    log::warn!("packet capture stopped: writer queue full");
                    if let Err(error) = capture.stop(false) {
                        log::error!("capture finalization failed: {error}");
                    }
                    let queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                    if !queue.expired && !queue.closed {
                        self.cancel.store(false, Ordering::Relaxed);
                    }
                }
                Work::Control(request, response) => {
                    let started = matches!(request, Request::Start(_));
                    let result = match request {
                        Request::Start(input) if !self.cancel.load(Ordering::Relaxed) => {
                            capture.start_input(input, Some(self.cancel.clone()))
                        }
                        Request::Start(_) => Ok(Vec::new()),
                        Request::Stop { disconnected } if disconnected && !capture.active() => {
                            Ok(Vec::new())
                        }
                        Request::Stop { disconnected } => {
                            capture.stop(disconnected).map(|event| vec![event])
                        }
                    };
                    let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                    let overloaded = queue
                        .work
                        .iter()
                        .any(|work| matches!(work, Work::Overloaded));
                    let cancelled = self.cancel.load(Ordering::Relaxed);
                    let closed = queue.closed;
                    if !overloaded && !closed {
                        self.cancel.store(false, Ordering::Relaxed);
                    }
                    if started && cancelled {
                        let _ = response.send(Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            "capture start cancelled",
                        )
                        .into()));
                    } else {
                        self.active
                            .store(capture.active() && !cancelled && !closed, Ordering::Relaxed);
                        let _ = response.send(result.map(|events| {
                            for event in events {
                                self.event(event);
                            }
                        }));
                    }
                    queue.pending = false;
                    queue.expired = false;
                    drop(queue);
                    if started && cancelled {
                        capture = CaptureController::new();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{FileWriteMode, PcapFile};

    #[test]
    fn stop_flushes_accepted_records_and_bad_replacement_keeps_capture() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("trace.pcap");
        let request = |path: &std::path::Path| PcapFileInfo {
            file: PcapFile::Path {
                path: path.to_str().unwrap().into(),
                mode: FileWriteMode::Overwrite,
            },
            max_bytes: None,
        };
        let capture = CaptureState::default();
        capture.start(request(&output)).unwrap();
        capture.record(&[0x45; 20]);
        assert!(capture.start(request(directory.path())).is_err());
        assert!(capture.active());
        capture.record(&[0x60; 40]);
        capture.stop(false).unwrap();
        assert!(!capture.active());
        assert!(matches!(
            capture.take_events().as_slice(),
            [
                Event::PacketCaptureStarted { .. },
                Event::PacketCaptureStopped {
                    reason: CaptureStopReason::Request { .. }
                },
            ]
        ));
        let bytes = std::fs::read(output).unwrap();
        assert_eq!(bytes.len(), 24 + 16 + 20 + 16 + 40);
        assert_eq!(&bytes[40..60], &[0x45; 20]);
        assert_eq!(&bytes[76..], &[0x60; 40]);
    }
}
