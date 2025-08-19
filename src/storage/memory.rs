use super::KeyValueStore;
use crate::error::StoreError;
use std::collections::HashMap;

/// Simple in-memory key-value store using HashMap
///
/// This is our "good enough" first implementation:
/// - Fast HashMap lookups
/// - No persistence (add later)
/// - No thread safety yet (add when needed for CLI integration)
/// - Basic pattern matching with * wildcards
#[derive(Debug, Clone)]
pub struct MemoryStore {
    data: HashMap<String, String>,
}

impl MemoryStore {
    /// Create a new empty store
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Helper function to check if a pattern matches a key
    /// For now, just supports * wildcard matching
    fn matches_pattern(key: &str, pattern: &str) -> bool {
        // TODO(human): Implement basic pattern matching
        // Handle these cases:
        // - "*" should match all keys
        // - "prefix*" should match keys starting with "prefix"
        // - "*suffix" should match keys ending with "suffix"
        // - "prefix*suffix" should match keys with prefix and suffix
        // - No wildcards should do exact matching

        // Hint: You can use string methods like starts_with(), ends_with()
        // and check for '*' characters in the pattern

        todo!("Implement pattern matching logic")
    }
}

impl KeyValueStore for MemoryStore {
    fn get(&self, key: &str) -> Result<String, StoreError> {
        // TODO(human): Implement get operation
        // Use self.data.get(key) and return appropriate Result
        // Remember: missing key should return StoreError::KeyNotFound

        match self.data.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(StoreError::KeyNotFound(key.to_string())),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StoreError> {
        // TODO(human): Implement set operation
        // Use self.data.insert(key, value)
        // For now, this should always succeed (return Ok(()))

        todo!("Implement set operation")
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        // TODO(human): Implement delete operation
        // Use self.data.remove(key) - it already returns Option<String>!
        // This is exactly what we want

        todo!("Implement delete operation")
    }

    fn keys(&self, pattern: Option<&str>) -> Vec<String> {
        // TODO(human): Implement keys listing
        // If pattern is None, return all keys
        // If pattern is Some(p), filter keys using matches_pattern helper
        // Use self.data.keys() and collect into Vec<String>

        todo!("Implement keys operation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_missing_key() {
        let store = MemoryStore::new();

        let result = store.get("missing_key");

        assert!(result.is_err());
        match result.unwrap_err() {
            StoreError::KeyNotFound(key) => assert_eq!(key, "missing_key"),
            _ => panic!("Expected KeyNotFound error"),
        }
    }

    #[test]
    fn test_get_existing_key() {
        let mut store = MemoryStore::new();

        // Manually insert a value to test GET
        // (We'll implement SET next, but for now let's test GET directly)
        store
            .data
            .insert("test_key".to_string(), "test_value".to_string());

        let result = store.get("test_key");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
    }
}
