use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let items = app
        .commands()
        .iter()
        .map(|cmd| {
            let content = cmd.command().to_string();
            ListItem::new(Line::from(content))
        })
        .collect::<Vec<_>>();

    let list = List::new(items).block(Block::default().title(" Command ").borders(Borders::ALL));

    frame.render_widget(list, area);
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
