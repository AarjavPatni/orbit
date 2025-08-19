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
        // TODO(human): Implement pattern matching for key filtering
        // Think about: What are the different wildcard scenarios we need to handle?
        // Consider: How can we break down pattern matching into logical cases?

        todo!("Implement pattern matching logic")
    }
}

impl KeyValueStore for MemoryStore {
    fn get(&self, key: &str) -> Result<String, StoreError> {
        match self.data.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(StoreError::KeyNotFound(key.to_string())),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StoreError> {
        self.data.insert(key, value);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        // TODO(human): Implement key removal
        // Think about: How do we remove a key from our data structure?
        // Consider: What should we return - the old value or just success/failure?

        todo!("Implement delete operation")
    }

    fn keys(&self, pattern: Option<&str>) -> Vec<String> {
        // TODO(human): Implement key enumeration with optional filtering
        // Think about: How do we get all keys from our data structure?
        // Consider: How should pattern filtering work? When should we filter?

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

    #[test]
    fn test_set_new_key() {
        let mut store = MemoryStore::new();

        let result = store.set("new_key".to_string(), "new_value".to_string());

        assert!(result.is_ok());

        // Verify the value was actually stored
        let get_result = store.get("new_key");
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), "new_value");
    }

    #[test]
    fn test_set_update_existing_key() {
        let mut store = MemoryStore::new();

        // Set initial value
        store
            .set("update_key".to_string(), "initial_value".to_string())
            .unwrap();

        // Update the value
        let result = store.set("update_key".to_string(), "updated_value".to_string());

        assert!(result.is_ok());

        // Verify the value was updated
        let get_result = store.get("update_key");
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), "updated_value");
    }
}
