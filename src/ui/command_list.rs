use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

pub fn render(frame: &mut Frame, area: Rect, _app: &App) {
    let output = Block::default().title(" Command ").borders(Borders::ALL);

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
            .expect("Failed to draw command list");
    }
}
