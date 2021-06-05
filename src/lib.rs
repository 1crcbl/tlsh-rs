//! # Trend Micro Locality Sensitive Hashing
//! This is a Rust port of TLSH algorithm. The crate can compute a hash value of a input byte array
//! and measure the difference between two hash values.
//!
//! The current implementation of TLSH has two conditions on the file size:
//! - the input must be at least 50 bytes long
//! - the size must not exceed 4Gb.
//!
//! ## Algorithm
//!
//! The algorithm to construct a TLSH digest is as follows (for more detail, see [J. Oliver et al.](https://documents.trendmicro.com/assets/wp/wp-locality-sensitive-hash.pdf)):
//! - **Step 1**: processes an input stream by using a sliding window of length 5 and populates the hash buckets.
//! Each triplet is passed through a hash function (in this implementation, the hash function is the  [Pearson hashing](https://en.wikipedia.org/wiki/Pearson_hashing)).
//! - **Step 2**: calculates the quartile points from the hash bucket obtained in step 1. This step might requires the sorting of the bucket array:
//! ```q1```: the lowest 25% of the array
//! ```q2```: the lowest 50% of the array
//! ```q3```: the lowest 75% of the array
//! - **Step 3**: computes the digest header. The first three bytes of a hash is reserved for the header. The header of a TLSH hash consists of three parts:
//! - The first byte is a checksum (with some modulo) of the byte string
//! - The second byte is computed from the logarithm of the byte string's length (with some modulo)
//! - The third byte is the result of ```q1_ratio <<< 4 | q2_ratio```, where  
//!     ```q1_ratio =  (q1 * 100 / q3) MOD 16```  
//!     ```q2_ratio =  (q2 * 100 / q3) MOD 16```  
//! - **Step 4**: constructs the digest body from the bucket array. Note: in this step, the reversing order in reading the bucket is assumed. This means, the last element is read first while the first is read last. Their value is converted into hex form and appended into the final hash value.
//!
//! ## Examples
//! To compute a hash value of a string, we will create an instance of [`TlshBuilder`]. After all
//! input data are pushed into the builder, we can construct an instance of [`Tlsh`]:
//! ```
//! use tlsh::{Tlsh, Version, BucketKind, ChecksumKind, TlshBuilder};
//!
//! let s1 = "Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...";
//! let mut builder = TlshBuilder::new(
//!    BucketKind::Bucket128,
//!    ChecksumKind::OneByte,
//!    crate::tlsh::Version::Version4,
//! );
//! builder.update(s1.as_bytes());
//! let tlsh1 = builder.build().unwrap();
//!
//! let s2 = "Morbi rhoncus ex mi, et iaculis erat euismod fermentum tincidunt";
//! let mut builder = TlshBuilder::new(
//!    BucketKind::Bucket128,
//!    ChecksumKind::OneByte,
//!    crate::tlsh::Version::Version4,
//! );
//! builder.update(s2.as_bytes());
//! let tlsh2 = builder.build().unwrap();
//!
//! // Calculate diff between s1 & s2, including length difference.
//! let _ = tlsh1.diff(&tlsh2, true);
//! // Calculate diff between s1 & s2, excluding length difference.
//! let _ = tlsh1.diff(&tlsh2, false);
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
