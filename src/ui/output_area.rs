use crate::model::{App, format_str};
use ansi_to_tui::IntoText;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let command = app.get_selected_command();

    let viewport_height = area.height.saturating_sub(2) as usize;
    let viewport_width = area.width.saturating_sub(2) as usize; // Account for borders

    let (text_lines, sub_offset) = if command.is_some() {
        let show_timestamp = app.command_log_timestamp_visibility();

        let (visible_lines, sub_offset) =
            app.get_visible_lines_with_sub_offset(viewport_height, viewport_width);

        let lines = visible_lines
            .iter()
            .flat_map(|line| {
                let content = format_str(line.timestamp(), line.content(), show_timestamp);

                content
                    .into_text()
                    .unwrap_or_else(|_| Text::from(content))
                    .lines
            })
            .collect();

        (lines, sub_offset)
    } else {
        (vec![Line::from("No command selected.")], 0)
    };

    let output = Paragraph::new(text_lines)
        .wrap(Wrap { trim: true })
        .scroll((sub_offset.min(u16::MAX as usize) as u16, 0))
        .block(Block::default().title(" Output ").borders(Borders::ALL));

    frame.render_widget(output, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::test_helpers::setup_test_terminal;

    #[test]
    fn test_render_output_area() {
        let app = App::new();
        let mut terminal = setup_test_terminal(80, 24);

        terminal
            .draw(|f| {
                let area = f.area();
                render(f, area, &app);
            })
            .expect("Failed to draw output area");
    }
}
