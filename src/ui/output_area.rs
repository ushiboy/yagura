use crate::model::{App, TimestampVisibility};
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let command = app.get_selected_command();

    let viewport_height = area.height.saturating_sub(2) as usize;

    let content = if let Some(cmd) = command {
        let total_lines = cmd.output_buffer().line_length();
        let scroll_offset = app
            .get_command_log_offset()
            .unwrap_or_else(|| total_lines.saturating_sub(viewport_height));

        cmd.output_buffer()
            .slice_lines(scroll_offset, viewport_height)
            .iter()
            .map(|line| {
                if app.command_log_timestamp_visibility() == &TimestampVisibility::Show {
                    Line::from(format!(
                        "[{}] {}",
                        line.timestamp().format("%H:%M:%S"),
                        line.content()
                    ))
                } else {
                    Line::from(line.content().to_string())
                }
            })
            .collect()
    } else {
        vec![Line::from("No command selected.")]
    };

    let output = Paragraph::new(content)
        .wrap(Wrap { trim: true })
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
