// A model representing the state of the command list UI component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandList {
    // The index of the currently selected command, if any.
    pub selected_command_index: Option<usize>,
}

impl CommandList {
    // Creates a new CommandList with no selected command.
    pub fn new() -> Self {
        Self {
            selected_command_index: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_command_list_new() {
        let command_list = CommandList::new();
        assert_eq!(command_list.selected_command_index, None);
    }
}
