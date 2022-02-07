use crate::Hasher;
use blake2::{digest::FixedOutput, Digest, Blake2s256};

/// This
/// is
/// a
/// fake
/// doc.
#[derive(Clone)]
pub struct Blake2sAlgorithm {}

impl Hasher for Blake2sAlgorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Blake2s256::new();

        hasher.update(data);
        <[u8; 32]>::from(hasher.finalize_fixed())
    }
}
