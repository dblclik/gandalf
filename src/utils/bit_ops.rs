use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitOpsError {
    ArrayLengthsNotEqual,
}

impl fmt::Display for BitOpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitOpsError::ArrayLengthsNotEqual => "Arrays must be of equal length!".fmt(f),
        }
    }
}

impl std::error::Error for BitOpsError {}

pub fn xor_bytes(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, BitOpsError> {
    if a.len() != b.len() {
        return Err(BitOpsError::ArrayLengthsNotEqual);
    }
    return Ok(a.iter().zip(b).map(|(x, y)| x ^ y).collect());
}
