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

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    /// Create a new empty store
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Validate a key according to basic rules
    fn validate_key(key: &str) -> Result<(), StoreError> {
        if key.is_empty() {
            return Err(StoreError::InvalidKey("Key cannot be empty".to_string()));
        }
        if key.len() > 512 {
            return Err(StoreError::InvalidKey(
                "Key too long (max 512 characters)".to_string(),
            ));
        }
        if key.contains('\0') {
            return Err(StoreError::InvalidKey(
                "Key cannot contain null bytes".to_string(),
            ));
        }
        Ok(())
    }

    /// Helper function to check if a pattern matches a key
    /// For now, just supports * wildcard matching
    fn matches_pattern(key: &str, pattern: &str) -> bool {
        // TODO(human): Implement pattern matching for key filtering
        // Think about: What are the different wildcard scenarios we need to handle?
        // Consider: How can we break down pattern matching into logical cases?

        let wildcard_matches: Vec<_> = pattern.match_indices('*').map(|(idx, _)| idx).collect();

        if wildcard_matches.len() == 1 {
            let wildcard_loc = wildcard_matches[0];

            if wildcard_loc == 0 {
                let query: &str = &pattern[1..];
                let query_matches: Vec<_> = key.match_indices(query).map(|(idx, _)| idx).collect();

                if !query_matches.contains(&(key.len() - query.len())) {
                    return false;
                }
            } else if wildcard_loc == pattern.len() - 1 {
                let query: &str = &pattern[..pattern.len() - 1];
                let query_matches: Vec<_> = key.match_indices(query).map(|(idx, _)| idx).collect();

                if !query_matches.contains(&0) {
                    return false;
                }
            } else {
                // TODO: Implement logic for wildcards anywhere in the pattern
                todo!()
            }

            true
        } else {
            // TODO: Implement logic for multiple wildcards
            todo!()
        }
    }
}

impl KeyValueStore for MemoryStore {
    fn get(&self, key: &str) -> Result<String, StoreError> {
        Self::validate_key(key)?;
        match self.data.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(StoreError::KeyNotFound(key.to_string())),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StoreError> {
        Self::validate_key(&key)?;
        self.data.insert(key, value);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<Option<String>, StoreError> {
        Self::validate_key(key)?;
        Ok(self.data.remove(key))
    }

    fn keys(&self, pattern: Option<&str>) -> Vec<String> {
        // TODO(human): Implement key enumeration with optional filtering
        // Think about: How do we get all keys from our data structure?
        // Consider: How should pattern filtering work? When should we filter?

        let pattern = pattern.unwrap_or("*");
        let mut matched_keys: Vec<String> = Vec::new();

        // TODO: Convert this to closure
        for k in self.data.keys() {
            if Self::matches_pattern(k, pattern) {
                matched_keys.push(k.clone());
            }
        }

        matched_keys
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

    #[test]
    fn test_delete_existing_key() {
        let mut store = MemoryStore::new();

        // Set up a key-value pair
        store
            .set("delete_me".to_string(), "goodbye".to_string())
            .unwrap();

        // Delete the key
        let result = store.delete("delete_me");

        // Should return Ok(Some(previous_value))
        assert!(result.is_ok());
        let deleted_value = result.unwrap();
        assert!(deleted_value.is_some());
        assert_eq!(deleted_value.unwrap(), "goodbye");

        // Verify the key is actually gone
        let get_result = store.get("delete_me");
        assert!(get_result.is_err());
        match get_result.unwrap_err() {
            StoreError::KeyNotFound(key) => assert_eq!(key, "delete_me"),
            _ => panic!("Expected KeyNotFound error"),
        }
    }

    #[test]
    fn test_delete_missing_key() {
        let mut store = MemoryStore::new();

        // Try to delete a key that doesn't exist
        let result = store.delete("nonexistent");

        // Should return Ok(None)
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_delete_after_multiple_operations() {
        let mut store = MemoryStore::new();

        // Set multiple keys
        store.set("key1".to_string(), "value1".to_string()).unwrap();
        store.set("key2".to_string(), "value2".to_string()).unwrap();
        store.set("key3".to_string(), "value3".to_string()).unwrap();

        // Delete one key
        let result = store.delete("key2");
        assert!(result.is_ok());
        let deleted_value = result.unwrap();
        assert!(deleted_value.is_some());
        assert_eq!(deleted_value.unwrap(), "value2");

        // Verify other keys still exist
        assert_eq!(store.get("key1").unwrap(), "value1");
        assert_eq!(store.get("key3").unwrap(), "value3");

        // Verify deleted key is gone
        assert!(store.get("key2").is_err());
    }

    #[test]
    fn test_keys_empty_store() {
        let store = MemoryStore::new();

        let keys = store.keys(None);

        assert_eq!(keys.len(), 0);
        assert_eq!(keys, Vec::<String>::new());
    }

    #[test]
    fn test_keys_with_data() {
        let mut store = MemoryStore::new();

        // Add some test data
        store
            .set("user:1".to_string(), "alice".to_string())
            .unwrap();
        store.set("user:2".to_string(), "bob".to_string()).unwrap();
        store
            .set("session:abc".to_string(), "active".to_string())
            .unwrap();

        let keys = store.keys(None);

        // Should return all 3 keys (order may vary due to HashMap)
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"user:1".to_string()));
        assert!(keys.contains(&"user:2".to_string()));
        assert!(keys.contains(&"session:abc".to_string()));
    }

    #[test]
    fn test_keys_with_pattern() {
        let mut store = MemoryStore::new();

        // Add test data
        store
            .set("user:1".to_string(), "alice".to_string())
            .unwrap();
        store.set("user:2".to_string(), "bob".to_string()).unwrap();
        store
            .set("session:abc".to_string(), "active".to_string())
            .unwrap();

        // Test prefix pattern matching
        let keys = store.keys(Some("user:*"));

        assert_eq!(keys.len(), 2); // Should only match user: prefixed keys
        assert!(keys.contains(&"user:1".to_string()));
        assert!(keys.contains(&"user:2".to_string()));
        assert!(!keys.contains(&"session:abc".to_string()));
    }

    #[test]
    fn test_matches_pattern_prefix_wildcard() {
        // Test patterns like "user:*" - should match keys that start with "user:"
        assert!(MemoryStore::matches_pattern("user:123", "user:*"));
        assert!(MemoryStore::matches_pattern("user:alice", "user:*"));
        assert!(MemoryStore::matches_pattern("user:", "user:*"));
        assert!(!MemoryStore::matches_pattern("session:abc", "user:*"));
        assert!(!MemoryStore::matches_pattern("admin:user:123", "user:*"));
    }

    #[test]
    fn test_matches_pattern_suffix_wildcard() {
        // Test patterns like "*:123" - should match keys that end with ":123"
        assert!(MemoryStore::matches_pattern("user:123", "*:123"));
        assert!(MemoryStore::matches_pattern("session:123", "*:123"));
        assert!(MemoryStore::matches_pattern(":123", "*:123"));
        assert!(!MemoryStore::matches_pattern("user:456", "*:123"));
        assert!(!MemoryStore::matches_pattern("user:123:extra", "*:123"));
    }

    #[test]
    fn test_matches_pattern_exact_match() {
        // When pattern has no wildcards, should match exactly
        assert!(MemoryStore::matches_pattern("user:123", "*"));
        assert!(MemoryStore::matches_pattern("any_key", "*"));
        assert!(MemoryStore::matches_pattern("", "*"));
    }

    #[test]
    fn test_matches_pattern_edge_cases() {
        // Empty strings and edge cases
        assert!(MemoryStore::matches_pattern("test", "test*"));
        assert!(MemoryStore::matches_pattern("test", "*test"));
        assert!(MemoryStore::matches_pattern("", "*"));
        assert!(MemoryStore::matches_pattern("x", "*x"));
        assert!(MemoryStore::matches_pattern("x", "x*"));
    }

    #[test]
    fn test_invalid_key_empty() {
        let store = MemoryStore::new();

        let result = store.get("");
        assert!(result.is_err());
        match result.unwrap_err() {
            StoreError::InvalidKey(msg) => assert_eq!(msg, "Key cannot be empty"),
            _ => panic!("Expected InvalidKey error"),
        }
    }

    #[test]
    fn test_invalid_key_too_long() {
        let mut store = MemoryStore::new();
        let long_key = "a".repeat(513); // Over the 512 character limit

        let result = store.set(long_key, "value".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            StoreError::InvalidKey(msg) => assert_eq!(msg, "Key too long (max 512 characters)"),
            _ => panic!("Expected InvalidKey error"),
        }
    }

    #[test]
    fn test_invalid_key_null_bytes() {
        let mut store = MemoryStore::new();

        let result = store.delete("key\0with\0nulls");
        assert!(result.is_err());
        match result.unwrap_err() {
            StoreError::InvalidKey(msg) => assert_eq!(msg, "Key cannot contain null bytes"),
            _ => panic!("Expected InvalidKey error"),
        }
    }

    #[test]
    fn test_validation_applied_to_all_operations() {
        let mut store = MemoryStore::new();
        let invalid_key = ""; // Empty key

        // Test GET
        assert!(store.get(invalid_key).is_err());

        // Test SET
        assert!(store
            .set(invalid_key.to_string(), "value".to_string())
            .is_err());

        // Test DELETE
        assert!(store.delete(invalid_key).is_err());
    }
}
