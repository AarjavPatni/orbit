use super::KeyValueStore;
use crate::error::StoreError;
use regex::Regex;
use std::collections::HashMap;

const REGEX_METACHARACTERS: &[char] = &[
    '.', '*', '+', '?', '^', '$', '|', '[', ']', '(', ')', '{', '}', '\\',
];

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

        // Only allow letters, numbers, and dashes
        for ch in key.chars() {
            if !ch.is_alphanumeric() && ch != '-' {
                return Err(StoreError::InvalidKey(format!(
                    "Key contains invalid character '{}'. Only letters, numbers, and dashes are allowed",
                    ch
                )));
            }
        }

        Ok(())
    }

    /// Helper function to check if a pattern matches a key
    /// For now, just supports * wildcard matching
    fn matches_pattern(key: &str, pattern: &str) -> bool {
        // TODO(human): Implement pattern matching for key filtering
        // Think about: What are the different wildcard scenarios we need to handle?
        // Consider: How can we break down pattern matching into logical cases?

        let should_auto_anchor =
            |pattern: &str| -> bool { !pattern.chars().any(|c| REGEX_METACHARACTERS.contains(&c)) };

        if should_auto_anchor(pattern) {
            pattern == key
        } else {
            let re: Regex = Regex::new(pattern).unwrap();

            re.is_match(key)
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

        let pattern = pattern.unwrap_or(".*");
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

        let result = store.get("missing-key");

        assert!(result.is_err());
        match result.unwrap_err() {
            StoreError::KeyNotFound(key) => assert_eq!(key, "missing-key"),
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
            .insert("test-key".to_string(), "test-value".to_string());

        let result = store.get("test-key");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-value");
    }

    #[test]
    fn test_set_new_key() {
        let mut store = MemoryStore::new();

        let result = store.set("new-key".to_string(), "new-value".to_string());

        assert!(result.is_ok());

        // Verify the value was actually stored
        let get_result = store.get("new-key");
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), "new-value");
    }

    #[test]
    fn test_set_update_existing_key() {
        let mut store = MemoryStore::new();

        // Set initial value
        store
            .set("update-key".to_string(), "initial-value".to_string())
            .unwrap();

        // Update the value
        let result = store.set("update-key".to_string(), "updated-value".to_string());

        assert!(result.is_ok());

        // Verify the value was updated
        let get_result = store.get("update-key");
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), "updated-value");
    }

    #[test]
    fn test_delete_existing_key() {
        let mut store = MemoryStore::new();

        // Set up a key-value pair
        store
            .set("delete-me".to_string(), "goodbye".to_string())
            .unwrap();

        // Delete the key
        let result = store.delete("delete-me");

        // Should return Ok(Some(previous_value))
        assert!(result.is_ok());
        let deleted_value = result.unwrap();
        assert!(deleted_value.is_some());
        assert_eq!(deleted_value.unwrap(), "goodbye");

        // Verify the key is actually gone
        let get_result = store.get("delete-me");
        assert!(get_result.is_err());
        match get_result.unwrap_err() {
            StoreError::KeyNotFound(key) => assert_eq!(key, "delete-me"),
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
            .set("user-1".to_string(), "alice".to_string())
            .unwrap();
        store.set("user-2".to_string(), "bob".to_string()).unwrap();
        store
            .set("session-abc".to_string(), "active".to_string())
            .unwrap();

        let keys = store.keys(None);

        // Should return all 3 keys (order may vary due to HashMap)
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"user-1".to_string()));
        assert!(keys.contains(&"user-2".to_string()));
        assert!(keys.contains(&"session-abc".to_string()));
    }

    #[test]
    fn test_keys_with_pattern() {
        let mut store = MemoryStore::new();

        // Add test data
        store
            .set("user-1".to_string(), "alice".to_string())
            .unwrap();
        store.set("user-2".to_string(), "bob".to_string()).unwrap();
        store
            .set("session-abc".to_string(), "active".to_string())
            .unwrap();

        // Test prefix pattern matching using regex
        let keys = store.keys(Some("user.*"));

        assert_eq!(keys.len(), 2); // Should only match user prefixed keys
        assert!(keys.contains(&"user-1".to_string()));
        assert!(keys.contains(&"user-2".to_string()));
        assert!(!keys.contains(&"session-abc".to_string()));
    }

    #[test]
    fn test_matches_pattern_prefix_wildcard() {
        // Test patterns like "^user-.*" - should match keys that start with "user-"
        assert!(MemoryStore::matches_pattern("user-123", "^user-.*"));
        assert!(MemoryStore::matches_pattern("user-alice", "^user-.*"));
        assert!(MemoryStore::matches_pattern("user-", "^user-.*"));
        assert!(!MemoryStore::matches_pattern("session-abc", "^user-.*"));
        assert!(!MemoryStore::matches_pattern("admin-user-123", "^user-.*"));
    }

    #[test]
    fn test_matches_pattern_suffix_wildcard() {
        // Test patterns like ".*-123$" - should match keys that end with "-123"
        assert!(MemoryStore::matches_pattern("user-123", ".*-123$"));
        assert!(MemoryStore::matches_pattern("session-123", ".*-123$"));
        assert!(MemoryStore::matches_pattern("-123", ".*-123$"));
        assert!(!MemoryStore::matches_pattern("user-456", ".*-123$"));
        assert!(!MemoryStore::matches_pattern("user-123-extra", ".*-123$"));
    }

    #[test]
    fn test_matches_pattern_match_all() {
        // Pattern ".*" should match any key
        assert!(MemoryStore::matches_pattern("user-123", ".*"));
        assert!(MemoryStore::matches_pattern("any-key", ".*"));
        assert!(MemoryStore::matches_pattern("", ".*"));
        assert!(MemoryStore::matches_pattern("a", ".*"));
    }

    #[test]
    fn test_matches_pattern_edge_cases() {
        // Empty strings and edge cases with proper regex
        assert!(MemoryStore::matches_pattern("test", "test.*")); // test followed by anything
        assert!(MemoryStore::matches_pattern("test", ".*test")); // anything followed by test
        assert!(MemoryStore::matches_pattern("", ".*")); // empty matches .*
        assert!(MemoryStore::matches_pattern("x", ".*x")); // anything followed by x
        assert!(MemoryStore::matches_pattern("x", "x.*")); // x followed by anything
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
            StoreError::InvalidKey(msg) => assert!(msg.contains("invalid character")),
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

    #[test]
    fn test_key_validation_invalid_characters() {
        let mut store = MemoryStore::new();

        // Test various invalid characters
        let invalid_keys = vec![
            "key:with:colons",
            "key.with.dots",
            "key with spaces",
            "key@with@symbols",
            "key/with/slashes",
            "key_with_underscores",
        ];

        for key in invalid_keys {
            let result = store.set(key.to_string(), "value".to_string());
            assert!(result.is_err(), "Key '{}' should be invalid", key);
            match result.unwrap_err() {
                StoreError::InvalidKey(_) => {} // Expected
                _ => panic!("Expected InvalidKey error for '{}'", key),
            }
        }
    }

    #[test]
    fn test_key_validation_valid_characters() {
        let mut store = MemoryStore::new();

        // Test valid characters (letters, numbers, dashes)
        let valid_keys = vec![
            "user123",
            "session-abc",
            "test-key-123",
            "ABC123",
            "a",
            "1",
            "user-session-123",
        ];

        for key in valid_keys {
            let result = store.set(key.to_string(), "value".to_string());
            assert!(result.is_ok(), "Key '{}' should be valid", key);
        }
    }

    #[test]
    fn test_pattern_matching_exact_strings() {
        // Test auto-anchoring for simple patterns (no regex metacharacters)
        assert!(MemoryStore::matches_pattern("test", "test"));
        assert!(MemoryStore::matches_pattern("user123", "user123"));
        assert!(MemoryStore::matches_pattern("session-abc", "session-abc"));

        // Should NOT match partial strings when auto-anchored
        assert!(!MemoryStore::matches_pattern("test123", "test"));
        assert!(!MemoryStore::matches_pattern("mytest", "test"));
        assert!(!MemoryStore::matches_pattern("user1234", "user123"));
    }

    #[test]
    fn test_pattern_matching_regex_patterns() {
        // Test regex patterns (contains metacharacters, no auto-anchoring)
        assert!(MemoryStore::matches_pattern("test123", "test.*"));
        assert!(MemoryStore::matches_pattern("test", "test.*"));
        assert!(MemoryStore::matches_pattern("testABC", "test.*"));

        assert!(MemoryStore::matches_pattern("user1", "user\\d+"));
        assert!(MemoryStore::matches_pattern("user123", "user\\d+"));
        assert!(!MemoryStore::matches_pattern("userABC", "user\\d+"));

        // Substring matching with regex (contains "prefix")
        assert!(MemoryStore::matches_pattern(
            "prefix-anything",
            ".*prefix.*"
        ));
        assert!(MemoryStore::matches_pattern(
            "anything-prefix-more",
            ".*prefix.*"
        ));
    }

    #[test]
    fn test_pattern_matching_edge_cases() {
        // Empty patterns and keys
        assert!(MemoryStore::matches_pattern("", ""));
        assert!(!MemoryStore::matches_pattern("test", ""));
        assert!(!MemoryStore::matches_pattern("", "test"));

        // Single characters
        assert!(MemoryStore::matches_pattern("a", "a"));
        assert!(!MemoryStore::matches_pattern("a", "b"));
        assert!(MemoryStore::matches_pattern("abc", ".")); // . matches any single char

        // Case sensitivity
        assert!(!MemoryStore::matches_pattern("Test", "test"));
        assert!(MemoryStore::matches_pattern("Test", "Test"));
    }
}
