use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct CommandLog {
    offset_by_command_id: HashMap<Uuid, usize>,
    timestamp_visibility: TimestampVisibility,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimestampVisibility {
    Show,
    Hide,
}

impl CommandLog {
    pub fn new() -> Self {
        Self {
            offset_by_command_id: HashMap::new(),
            timestamp_visibility: TimestampVisibility::Show,
        }
    }

    pub fn get_offset(&self, command_id: &Uuid) -> Option<usize> {
        self.offset_by_command_id.get(command_id).cloned()
    }

    pub fn set_offset(&mut self, command_id: Uuid, offset: usize) {
        self.offset_by_command_id.insert(command_id, offset);
    }

    pub fn remove_offset(&mut self, command_id: &Uuid) {
        self.offset_by_command_id.remove(command_id);
    }

    pub fn toggle_timestamp_visibility(&mut self) {
        self.timestamp_visibility = match self.timestamp_visibility {
            TimestampVisibility::Show => TimestampVisibility::Hide,
            TimestampVisibility::Hide => TimestampVisibility::Show,
        };
    }

    pub fn timestamp_visibility(&self) -> &TimestampVisibility {
        &self.timestamp_visibility
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_log_new() {
        let command_log = CommandLog::new();
        assert!(command_log.offset_by_command_id.is_empty());
    }

    #[test]
    fn test_set_and_get_offset() {
        let mut command_log = CommandLog::new();
        let command_id = Uuid::now_v7();

        command_log.set_offset(command_id, 5);

        assert_eq!(command_log.get_offset(&command_id), Some(5));
    }

    #[test]
    fn test_remove_offset() {
        let mut command_log = CommandLog::new();
        let command_id = Uuid::now_v7();
        command_log.set_offset(command_id, 10);

        command_log.remove_offset(&command_id);

        assert_eq!(command_log.get_offset(&command_id), None);
    }
}
