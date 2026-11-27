"""Add a temporary managed-capture UDP echo measurement to the Linux TUN fixture."""
from pathlib import Path

path = Path("tests/linux_tun.rs")
source = path.read_text()
start = source.index("fn tracked_customer_reply_outside_virtual_subnet_survives_the_real_tun_pipeline")
before, fixture = source[:start], source[start:]
point = "    let mut pipe = [-1; 2];"
measurement = """
    if std::env::var_os("PROTUNNEXT_HEALTHY_CAPTURE_MEASUREMENT").is_some() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("healthy.pcap");
        connection.start_packet_capture(api::PcapFileInfo {
            file: api::PcapFile::Path {
                path: path.to_str().unwrap().into(),
                mode: api::FileWriteMode::Overwrite,
            },
            max_bytes: None,
        });
        assert!(matches!(events.recv_timeout(Duration::from_secs(3)).unwrap(),
            api::Event::PacketCaptureStarted { .. }));
        const ROUNDS: usize = 4000;
        let mut payload = [0x5a; 1024];
        let mut reply = [0; 1024];
        let started = std::time::Instant::now();
        for sequence in 0..ROUNDS {
            payload[..8].copy_from_slice(&(sequence as u64).to_le_bytes());
            app.send_to(&payload, "192.0.2.100:9000").unwrap();
            let (n, _) = app.recv_from(&mut reply).unwrap();
            assert_eq!(&reply[..n], &payload);
        }
        let elapsed = started.elapsed();
        connection.disconnect_and_wait();
        assert!(matches!(events.recv_timeout(Duration::from_secs(3)).unwrap(),
            api::Event::PacketCaptureStopped { reason: api::CaptureStopReason::Disconnected { .. } }));
        let bytes = std::fs::read(&path).unwrap();
        let mut offset = 24;
        let mut records = 0;
        while offset < bytes.len() {
            let size = u32::from_le_bytes(bytes[offset+8..offset+12].try_into().unwrap());
            offset += 16 + size as usize;
            records += 1;
        }
        assert_eq!(offset, bytes.len(), "PCAP has a partial record");
        assert!(records >= 4 * ROUNDS, "missing captured packets");
        println!("HEALTHY_CAPTURE {} {} {}", ROUNDS, elapsed.as_micros(), records);
        stop.store(true, Ordering::Relaxed);
        peer.join().unwrap();
        return;
    }
"""
assert fixture.count(point) == 1
path.write_text(before + fixture.replace(point, measurement + point, 1))
