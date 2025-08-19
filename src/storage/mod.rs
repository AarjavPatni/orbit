pub mod memory;

use crate::error::StoreError;

/// Core trait for key-value storage operations
///
/// Design philosophy:
/// - GET: Returns error for missing keys since caller depends on the value
/// - SET: Simple storage, errors only on system failures  
/// - DELETE: Returns Option<String> - gives info without "false success" problem
/// - KEYS: Simple pattern matching with * wildcards for now
pub trait KeyValueStore {
    /// Get a value by key
    /// Returns StoreError::KeyNotFound if key doesn't exist
    fn get(&self, key: &str) -> Result<String, StoreError>;

    /// Set a key-value pair
    /// Only fails on system errors (memory, etc.)
    fn set(&mut self, key: String, value: String) -> Result<(), StoreError>;

    /// Delete a key
    /// Returns Some(old_value) if key existed, None if it didn't
    /// This is an idempotent operation - "success" means key doesn't exist
    fn delete(&mut self, key: &str) -> Option<String>;

    /// List keys matching a pattern
    /// Pattern supports basic * wildcards for now
    /// None pattern returns all keys
    fn keys(&self, pattern: Option<&str>) -> Vec<String>;
}
