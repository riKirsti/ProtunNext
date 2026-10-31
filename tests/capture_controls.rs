use protun_next::api::{capture::CaptureController, types::*};
use std::time::Duration;
#[cfg(unix)]
#[test]
fn capture_owned_descriptors_support_write_only_files_and_nonseekable_streams() {
    use std::{
        fs::File,
        io::Read,
        os::{fd::IntoRawFd, unix::net::UnixStream},
    };
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("write-only.pcap");
    let output = File::options()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    let mut capture = CaptureController::new();
    capture
        .start(PcapFileInfo {
            file: PcapFile::Fd(output.into_raw_fd()),
            max_bytes: None,
        })
        .unwrap();
    capture.packet(Duration::from_secs(1), &[0x45; 20]).unwrap();
    capture.stop(false).unwrap();
    assert_eq!(std::fs::read(&path).unwrap().len(), 60);
    let (output, mut input) = UnixStream::pair().unwrap();
    input
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    capture
        .start(PcapFileInfo {
            file: PcapFile::Fd(output.into_raw_fd()),
            max_bytes: None,
        })
        .unwrap();
    capture.packet(Duration::from_secs(2), &[0x60; 40]).unwrap();
    capture.stop(false).unwrap();
    let mut bytes = vec![];
    input.read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes.len(), 80);
    assert_eq!(&bytes[20..24], &101u32.to_le_bytes());
}
#[test]
fn capture_append_overwrite_limits_and_stop_events_use_real_files() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("capture.pcap");
    let request = |mode, max_bytes| PcapFileInfo {
        file: PcapFile::Path {
            path: path.to_str().unwrap().to_owned(),
            mode,
        },
        max_bytes,
    };
    let mut capture = CaptureController::new();
    assert!(matches!(
        capture
            .start(request(FileWriteMode::Overwrite, Some(60)))
            .unwrap()
            .as_slice(),
        [Event::PacketCaptureStarted { .. }]
    ));
    assert!(
        capture
            .packet(Duration::from_secs(1), &[0x45; 20])
            .unwrap()
            .is_none()
    );
    assert!(matches!(
        capture.packet(Duration::from_secs(2), &[0x45; 20]).unwrap(),
        Some(Event::PacketCaptureStopped {
            reason: CaptureStopReason::MaxSizeReached { .. }
        })
    ));
    assert!(!capture.active());
    assert_eq!(std::fs::metadata(&path).unwrap().len(), 60);
    capture.start(request(FileWriteMode::Append, None)).unwrap();
    capture.packet(Duration::from_secs(3), &[0x46; 20]).unwrap();
    assert!(matches!(
        capture.stop(false).unwrap(),
        Event::PacketCaptureStopped {
            reason: CaptureStopReason::Request { .. }
        }
    ));
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.len(), 96);
    assert_eq!(&bytes[76..], &[0x46; 20]);
    capture
        .start(request(FileWriteMode::Overwrite, None))
        .unwrap();
    assert!(matches!(
        capture.stop(true).unwrap(),
        Event::PacketCaptureStopped {
            reason: CaptureStopReason::Disconnected { .. }
        }
    ));
    assert_eq!(std::fs::metadata(&path).unwrap().len(), 24);
    assert!(matches!(
        capture.stop(false).unwrap(),
        Event::PacketCaptureStopped {
            reason: CaptureStopReason::AlreadyStopped
        }
    ));
}
#[cfg(unix)]
#[test]
fn invalid_capture_replacement_keeps_running_sink_and_symlinks_are_refused() {
    use std::os::unix::fs::symlink;
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("original");
    let alias = directory.path().join("alias");
    let mut capture = CaptureController::new();
    let info = |path: &std::path::Path| PcapFileInfo {
        file: PcapFile::Path {
            path: path.to_str().unwrap().to_owned(),
            mode: FileWriteMode::Overwrite,
        },
        max_bytes: None,
    };
    capture.start(info(&original)).unwrap();
    symlink(&original, &alias).unwrap();
    assert!(capture.start(info(&alias)).is_err());
    assert!(capture.active());
    capture.packet(Duration::ZERO, &[0x45; 20]).unwrap();
    capture.stop(false).unwrap();
    assert_eq!(std::fs::metadata(original).unwrap().len(), 60);
}
