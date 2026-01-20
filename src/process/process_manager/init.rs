use std::collections::HashMap;

use super::ProcessManager;

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_process_manager() {
        let manager = ProcessManager::new();

        assert_eq!(
            manager.handlers.len(),
            0,
            "New ProcessManager should have no processes"
        );
    }
}
