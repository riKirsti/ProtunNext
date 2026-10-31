//! Optional PCAP writer for raw IP packets. Disabled unless a caller supplies a writer.
use std::io::{self, Write};
use std::time::Duration;
pub struct Capture<W: Write> {
    writer: W,
    written: u64,
    limit: u64,
}
impl<W: Write> Capture<W> {
    pub fn new(mut writer: W, limit: u64) -> io::Result<Self> {
        if limit < 24 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "capture limit"));
        }
        writer.write_all(&0xa1b2c3d4u32.to_le_bytes())?;
        writer.write_all(&2u16.to_le_bytes())?;
        writer.write_all(&4u16.to_le_bytes())?;
        writer.write_all(&[0; 8])?;
        writer.write_all(&65535u32.to_le_bytes())?;
        writer.write_all(&101u32.to_le_bytes())?;
        Ok(Self {
            writer,
            written: 24,
            limit,
        })
    }
    pub fn packet(&mut self, wall: Duration, packet: &[u8]) -> io::Result<bool> {
        if packet.len() > 65535 || wall.as_secs() > u32::MAX as u64 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "pcap packet"));
        }
        let size = 16 + packet.len() as u64;
        if self.written.saturating_add(size) > self.limit {
            return Ok(false);
        }
        self.writer
            .write_all(&(wall.as_secs() as u32).to_le_bytes())?;
        self.writer.write_all(&wall.subsec_micros().to_le_bytes())?;
        self.writer
            .write_all(&(packet.len() as u32).to_le_bytes())?;
        self.writer
            .write_all(&(packet.len() as u32).to_le_bytes())?;
        self.writer.write_all(packet)?;
        self.written += size;
        Ok(true)
    }
    pub fn finish(mut self) -> io::Result<W> {
        self.writer.flush()?;
        Ok(self.writer)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stops_at_boundary_without_partial_record() {
        let mut c = Capture::new(Vec::new(), 60).unwrap();
        assert!(c.packet(Duration::from_secs(1), &[0x45; 20]).unwrap());
        assert!(!c.packet(Duration::from_secs(2), &[0x45; 20]).unwrap());
        let bytes = c.finish().unwrap();
        assert_eq!(bytes.len(), 60);
        assert_eq!(&bytes[20..24], &101u32.to_le_bytes());
    }
}
