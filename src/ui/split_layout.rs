use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::model::App;

pub fn split_layout(frame: &mut Frame, app: &App) -> Rc<[Rect]> {
    let help_bar_height = if app.help_visible() { 1 } else { 0 };
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(help_bar_height),
            Constraint::Fill(1),
            Constraint::Length(help_bar_height),
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

        let app = App::new();
        terminal
            .draw(|f| {
                let chunks = split_layout(f, &app);

                assert_eq!(chunks.len(), 4);
                assert!(chunks[0].height <= 6);
                assert!(chunks[1].height == 1);
                assert!(chunks[2].height >= 17);
                assert!(chunks[2].height <= 22);
                assert!(chunks[3].height == 1);
            })
            .expect("Failed to test layout");
    }

    #[test]
    fn test_split_layout_with_help() {
        let mut terminal = setup_test_terminal(80, 24);

        let mut app = App::new();
        app.toggle_help(); // to hide

        terminal
            .draw(|f| {
                let chunks = split_layout(f, &app);

                assert_eq!(chunks.len(), 4);
                assert!(chunks[0].height <= 6);
                assert!(chunks[1].height == 0);
                assert!(chunks[2].height >= 17);
                assert!(chunks[2].height <= 22);
                assert!(chunks[3].height == 0);
            })
            .expect("Failed to test layout with help");
    }
}
