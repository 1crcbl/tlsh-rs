use std::num::ParseIntError;

/// An enum for possible errors that might occur while calculating hash values.
#[derive(Debug)]
pub enum TlshError {
    ///
    DataLenOverflow,
    /// The hash string is malformed and cannot be parsed.
    InvalidHashValue,
    /// Fails to parse a hex string to integer.
    ParseHexFailed,
}

impl From<ParseIntError> for TlshError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseHexFailed
    }
}
