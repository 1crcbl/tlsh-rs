//! # Trend Micro Locality Sensitive Hashing
//!
//! ## Algorithm
//! alg
//!
//! ## Distance
//!
//! ## Examples
//! blah
mod helper;

mod error;
pub use error::TlshError;

mod tlsh;
pub use crate::tlsh::BucketKind;
pub use crate::tlsh::ChecksumKind;
pub use crate::tlsh::Tlsh;
pub use crate::tlsh::TlshBuilder;
pub use crate::tlsh::Version;

mod tests;
