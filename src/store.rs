use std::collections::HashMap;

/// Store (σ) mapping signals/coils to boolean values
/// Supports parameterised signals/coils using key format: "name(param1,param2)"
#[derive(Debug, Clone, Default)]
pub struct Store {
    /// Signal/coil values: name -> bool
    values: HashMap<String, bool>,
}

impl Store {
    /// Create a new empty store
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Get value for signal/coil
    pub fn get(&self, name: &str) -> Option<bool> {
        self.values.get(name).copied()
    }

    /// Set value for signal/coil
    pub fn set(&mut self, name: String, value: bool) {
        self.values.insert(name, value);
    }

    /// Check if signal/coil exists
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Merge environment inputs (ι) into store
    /// Signals are updated, coils retain their values
    pub fn merge_inputs(&mut self, inputs: &HashMap<String, bool>, signal_names: &[String]) {
        for (name, value) in inputs {
            // Only update signals, not coils
            if signal_names.contains(name) {
                self.values.insert(name.clone(), *value);
            }
        }
    }

    /// Get all coil values (for output)
    pub fn get_coils(&self, coil_names: &[String]) -> HashMap<String, bool> {
        coil_names
            .iter()
            .filter_map(|name| {
                self.values.get(name).map(|&value| (name.clone(), value))
            })
            .collect()
    }

    /// Clone the store
    pub fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_basic() {
        let mut store = Store::new();
        store.set("signal1".to_string(), true);
        assert_eq!(store.get("signal1"), Some(true));
        assert_eq!(store.get("signal2"), None);
    }

    #[test]
    fn test_store_merge() {
        let mut store = Store::new();
        store.set("signal1".to_string(), false);
        store.set("coil1".to_string(), false);

        let mut inputs = HashMap::new();
        inputs.insert("signal1".to_string(), true);
        inputs.insert("coil1".to_string(), true);

        let signal_names = vec!["signal1".to_string()];
        store.merge_inputs(&inputs, &signal_names);

        assert_eq!(store.get("signal1"), Some(true)); // Updated
        assert_eq!(store.get("coil1"), Some(false)); // Not updated (not a signal)
    }

    #[test]
    fn test_store_get_coils() {
        let mut store = Store::new();
        store.set("coil1".to_string(), true);
        store.set("coil2".to_string(), false);
        store.set("signal1".to_string(), true);

        let coil_names = vec!["coil1".to_string(), "coil2".to_string()];
        let coils = store.get_coils(&coil_names);

        assert_eq!(coils.get("coil1"), Some(&true));
        assert_eq!(coils.get("coil2"), Some(&false));
        assert_eq!(coils.get("signal1"), None); // Not a coil
        assert_eq!(coils.len(), 2);
    }

    #[test]
    fn test_store_contains() {
        let mut store = Store::new();
        store.set("signal1".to_string(), true);

        assert!(store.contains("signal1"));
        assert!(!store.contains("signal2"));
    }
}
