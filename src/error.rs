use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StoreError {
    KeyNotFound(String),
    InvalidKey(String),
    StorageFailure(String),
    LockError(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            StoreError::InvalidKey(key) => write!(f, "Invalid key: {}", key),
            StoreError::StorageFailure(msg) => write!(f, "Storage failure: {}", msg),
            StoreError::LockError(msg) => write!(f, "Lock error: {}", msg),
        }
    }
}

impl std::error::Error for StoreError {}
