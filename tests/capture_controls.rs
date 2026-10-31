use protun_next::api::{capture::CaptureController, types::*};
use std::time::Duration;
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
#[cfg(target_os = "linux")]
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
