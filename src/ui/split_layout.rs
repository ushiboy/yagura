use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn split_layout(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(frame.area())
}

#[cfg(test)]
mod tests {
    use crate::ui::test_helpers::setup_test_terminal;

    use super::*;

    #[test]
    fn test_split_layout() {
        let mut terminal = setup_test_terminal(80, 24);

        terminal
            .draw(|f| {
                let chunks = split_layout(f);

                assert_eq!(chunks.len(), 4);
                assert!(chunks[0].height <= 6);
                assert!(chunks[1].height == 1);
                assert!(chunks[2].height >= 17);
                assert!(chunks[2].height <= 22);
                assert!(chunks[3].height == 1);
            })
            .expect("Failed to test layout");
    }
}
