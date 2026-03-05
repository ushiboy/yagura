use crate::model::{App, OutputLine};
use ansi_to_tui::IntoText;
use chrono::{DateTime, Local};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use unicode_width::UnicodeWidthStr;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let command = app.get_selected_command();

    let viewport_height = area.height.saturating_sub(2) as usize;
    let viewport_width = area.width.saturating_sub(2) as usize; // Account for borders

    let content = if let Some(cmd) = command {
        let total_lines = cmd.output_buffer().line_length();
        let scroll_offset = app
            .get_command_log_offset()
            .unwrap_or_else(|| total_lines.saturating_sub(viewport_height));

        let show_timestamp = app.command_log_timestamp_visibility();

        let sliced_lines = cmd
            .output_buffer()
            .slice_lines(scroll_offset, viewport_height);

        let filtered_lines = filter_lines_by_physical_height(
            sliced_lines,
            viewport_width,
            viewport_height,
            show_timestamp,
        );

        filtered_lines
            .iter()
            .flat_map(|line| {
                let content = format_str(line.timestamp(), line.content(), show_timestamp);

                content
                    .into_text()
                    .unwrap_or_else(|_| Text::from(content))
                    .lines
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

fn filter_lines_by_physical_height(
    lines: Vec<&OutputLine>,
    viewport_width: usize,
    viewport_height: usize,
    show_timestamp: bool,
) -> Vec<&OutputLine> {
    if viewport_height == 0 || lines.is_empty() {
        return Vec::new();
    }

    let mut accumulated_height = 0;
    let mut result = Vec::new();

    for line in lines.iter().rev() {
        let content = format_str(line.timestamp(), &line.plain_text(), show_timestamp);

        let physical_lines = calculate_physical_lines(&content, viewport_width);

        if accumulated_height + physical_lines <= viewport_height {
            accumulated_height += physical_lines;
            result.push(*line);
        } else {
            break;
        }
    }

    result.reverse();
    result
}

fn calculate_physical_lines(text: &str, viewport_width: usize) -> usize {
    if viewport_width == 0 {
        return 0;
    }

    let text_width = UnicodeWidthStr::width(text);

    if text_width == 0 {
        return 1;
    }

    text_width.div_ceil(viewport_width)
}

fn format_str(timestamp: &DateTime<Local>, str: &str, show_timestamp: bool) -> String {
    if show_timestamp {
        format!("[{}] {}", timestamp.format("%H:%M:%S"), str)
    } else {
        str.to_string()
    }
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
