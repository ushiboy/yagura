use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect, _app: &App) {
    let command = _app.get_selected_command();

    let content = if let Some(cmd) = command {
        let lines: Vec<Line> = cmd
            .output_lines()
            .iter()
            .map(|line| Line::from(format!("{}", line.content())))
            .collect();

        lines
    } else {
        vec![Line::from("No command selected.")]
    };

    let output =
        Paragraph::new(content).block(Block::default().title(" Output ").borders(Borders::ALL));

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
