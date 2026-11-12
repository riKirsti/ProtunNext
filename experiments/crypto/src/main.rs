//! Synthetic preselection only: this is not the production packet/session path.
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{AeadInPlace, KeyInit},
};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

trait Key: Sized {
    fn new(bytes: [u8; 32]) -> Self;
    fn seal(&self, nonce: [u8; 12], aad: &[u8], buffer: &mut [u8]) -> Result<usize, ()>;
    fn open(&self, nonce: [u8; 12], aad: &[u8], buffer: &mut [u8]) -> Result<usize, ()>;
}
struct Ring(ring::aead::LessSafeKey);
struct AwsLc(aws_lc_rs::aead::LessSafeKey);
struct RustCrypto(ChaCha20Poly1305);

macro_rules! compatible_key {
    ($kind:ident, $module:path) => {
        impl Key for $kind {
            fn new(bytes: [u8; 32]) -> Self {
                use $module as aead;
                Self(aead::LessSafeKey::new(
                    aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &bytes).unwrap(),
                ))
            }
            fn seal(&self, nonce: [u8; 12], aad: &[u8], buffer: &mut [u8]) -> Result<usize, ()> {
                use $module as aead;
                let len = buffer.len().checked_sub(16).ok_or(())?;
                let tag = self
                    .0
                    .seal_in_place_separate_tag(
                        aead::Nonce::assume_unique_for_key(nonce),
                        aead::Aad::from(aad),
                        &mut buffer[..len],
                    )
                    .map_err(|_| ())?;
                buffer[len..].copy_from_slice(tag.as_ref());
                Ok(len + 16)
            }
            fn open(&self, nonce: [u8; 12], aad: &[u8], buffer: &mut [u8]) -> Result<usize, ()> {
                use $module as aead;
                self.0
                    .open_in_place(
                        aead::Nonce::assume_unique_for_key(nonce),
                        aead::Aad::from(aad),
                        buffer,
                    )
                    .map(|s| s.len())
                    .map_err(|_| ())
            }
        }
    };
}
compatible_key!(Ring, ring::aead);
compatible_key!(AwsLc, aws_lc_rs::aead);
impl Key for RustCrypto {
    fn new(bytes: [u8; 32]) -> Self {
        Self(ChaCha20Poly1305::new(&bytes.into()))
    }
    fn seal(&self, nonce: [u8; 12], aad: &[u8], buffer: &mut [u8]) -> Result<usize, ()> {
        let len = buffer.len().checked_sub(16).ok_or(())?;
        let tag = self
            .0
            .encrypt_in_place_detached(&nonce.into(), aad, &mut buffer[..len])
            .map_err(|_| ())?;
        buffer[len..].copy_from_slice(&tag);
        Ok(len + 16)
    }
    fn open(&self, nonce: [u8; 12], aad: &[u8], buffer: &mut [u8]) -> Result<usize, ()> {
        let len = buffer.len().checked_sub(16).ok_or(())?;
        let (body, tag) = buffer.split_at_mut(len);
        let tag: &[u8; 16] = (&*tag).try_into().map_err(|_| ())?;
        self.0
            .decrypt_in_place_detached(&nonce.into(), aad, body, tag.into())
            .map_err(|_| ())?;
        Ok(len)
    }
}
fn nonce(counter: u64) -> [u8; 12] {
    let mut n = [0; 12];
    n[4..].copy_from_slice(&counter.to_le_bytes());
    n
}
fn body(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 251) as u8).collect()
}

fn measure<K: Key>(backend: &str, size: usize, operation: &str, duration: Duration) {
    // Public synthetic key/data only; no live traffic, session or replay state.
    let key = K::new([7; 32]);
    let plain = body(size);
    let mut cipher = vec![0; size + 16];
    cipher[..size].copy_from_slice(&plain);
    Ring::new([7; 32]).seal(nonce(0), &[], &mut cipher).unwrap();
    let mut buffer = cipher.clone();
    let mut iterations = 0u64;
    let started = Instant::now();
    loop {
        for _ in 0..64 {
            let len = if operation == "open" {
                buffer.copy_from_slice(&cipher);
                key.open(nonce(0), &[], black_box(&mut buffer)).unwrap()
            } else {
                buffer[..size].copy_from_slice(&plain);
                key.seal(nonce(iterations), &[], black_box(&mut buffer))
                    .unwrap()
            };
            black_box(len);
            iterations += 1;
        }
        if started.elapsed() >= duration {
            break;
        }
    }
    let seconds = started.elapsed().as_secs_f64();
    if operation == "seal" {
        let len = Ring::new([7; 32])
            .open(nonce(iterations - 1), &[], &mut buffer)
            .unwrap();
        assert_eq!(&buffer[..len], plain);
    } else {
        assert_eq!(&buffer[..size], plain);
    }
    println!(
        "{}",
        serde_json::json!({"backend":backend,"operation":operation,"payload_bytes":size,"iterations":iterations,"seconds":seconds,"ns_per_operation":seconds*1e9/iterations as f64,"verified":true,"includes_input_restore":true})
    );
}
fn run<K: Key>(backend: &str, duration: Duration) {
    for size in [0, 64, 256, 1280, 1420, 16384] {
        for op in ["open", "seal"] {
            measure::<K>(backend, size, op, duration);
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "usage: stage5-crypto-screen ring|rustcrypto|aws-lc milliseconds-per-cell"
    );
    let duration = Duration::from_millis(args[2].parse().unwrap());
    assert!((10..=10000).contains(&duration.as_millis()));
    match args[1].as_str() {
        "ring" => run::<Ring>("ring", duration),
        "rustcrypto" => run::<RustCrypto>("rustcrypto", duration),
        "aws-lc" => run::<AwsLc>("aws-lc", duration),
        _ => panic!("unknown backend"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hex(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks_exact(2)
            .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
            .collect()
    }
    fn known_answer<K: Key>() {
        // RFC 8439 section 2.8.2, https://www.rfc-editor.org/rfc/rfc8439#section-2.8.2
        let key = K::new(std::array::from_fn(|i| 0x80 + i as u8));
        let n = hex("070000004041424344454647").try_into().unwrap();
        let aad = hex("50515253c0c1c2c3c4c5c6c7");
        let plain=b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        let expected = hex(concat!(
            "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6",
            "3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36",
            "92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc",
            "3ff4def08e4b7a9de576d26586cec64b61161ae10b594f09e26a7e902ecbd0600691"
        ));
        let mut buffer = plain.to_vec();
        buffer.resize(plain.len() + 16, 0);
        key.seal(n, &aad, &mut buffer).unwrap();
        assert_eq!(buffer, expected);
        assert_eq!(key.open(n, &aad, &mut buffer).unwrap(), plain.len());
        assert_eq!(&buffer[..plain.len()], plain);
    }
    #[test]
    fn rfc8439_known_answers_all_backends() {
        known_answer::<Ring>();
        known_answer::<RustCrypto>();
        known_answer::<AwsLc>();
    }
    fn interop<S: Key, R: Key>() {
        let sender = S::new([7; 32]);
        let receiver = R::new([7; 32]);
        for size in [0, 1, 15, 16, 17, 64, 255, 256, 1280, 1420, 16384, 65503] {
            let plain = body(size);
            let mut cipher = plain.clone();
            cipher.resize(size + 16, 0);
            sender.seal(nonce(size as u64), &[], &mut cipher).unwrap();
            for index in [0, cipher.len() - 1] {
                let mut bad = cipher.clone();
                bad[index] ^= 1;
                assert!(receiver.open(nonce(size as u64), &[], &mut bad).is_err());
            }
            let mut bad_nonce = cipher.clone();
            assert!(
                receiver
                    .open(nonce(size as u64 + 1), &[], &mut bad_nonce)
                    .is_err()
            );
            let mut bad_aad = cipher.clone();
            assert!(
                receiver
                    .open(nonce(size as u64), &[1], &mut bad_aad)
                    .is_err()
            );
            let len = receiver.open(nonce(size as u64), &[], &mut cipher).unwrap();
            assert_eq!(&cipher[..len], plain);
        }
        for size in 0..16 {
            assert!(receiver.open(nonce(0), &[], &mut vec![0; size]).is_err());
        }
    }
    #[test]
    fn cross_backend_packet_sizes_and_authentication_failures() {
        interop::<Ring, RustCrypto>();
        interop::<RustCrypto, Ring>();
        interop::<Ring, AwsLc>();
        interop::<AwsLc, Ring>();
        interop::<RustCrypto, AwsLc>();
        interop::<AwsLc, RustCrypto>();
    }
}
