use super::list_item::list_item;
use crate::model::App;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List},
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let inner_height = area.height as usize;
    let offset = app.command_list_scroll_offset();
    let inner_width = area.width as usize;
    let max_len = if inner_width > 50 {
        inner_width - 50
    } else {
        30
    };

    let items = app
        .commands()
        .iter()
        .enumerate()
        .skip(offset)
        .take(inner_height)
        .map(|(i, cmd)| list_item(cmd, app.selected_command_index() == Some(i), max_len))
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
