//! This module contains built-in implementations of the [`Hasher`]
//!
//! [`Hasher`]: crate::Hasher
mod sha256;
mod blake2s;

pub use sha256::Sha256Algorithm as Sha256;
pub use blake2s::Blake2sAlgorithm as Blake2s;
