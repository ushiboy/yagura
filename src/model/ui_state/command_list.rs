// A model representing the state of the command list UI component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandList {
    // The index of the currently selected command, if any.
    selected_command_index: Option<usize>,
}

impl CommandList {
    // Creates a new CommandList with no selected command.
    pub fn new() -> Self {
        Self {
            selected_command_index: None,
        }
    }

    pub fn selected_command_index(&self) -> Option<usize> {
        self.selected_command_index
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_command_index = Some(index);
    }

    pub fn clear_selection(&mut self) {
        self.selected_command_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_command_list_new() {
        let command_list = CommandList::new();
        assert_eq!(command_list.selected_command_index(), None);
    }

    #[test]
    fn test_set_and_get_selected_command_index() {
        let mut command_list = CommandList::new();

        command_list.set_selected_index(1);

        assert_eq!(command_list.selected_command_index(), Some(1));
    }

    #[test]
    fn test_clear_selection() {
        let mut command_list = CommandList::new();
        command_list.set_selected_index(2);

        command_list.clear_selection();

        assert_eq!(command_list.selected_command_index(), None);
    }
}
