use crate::model::App;
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: Rect, _app: &App) {
    let debug_text = vec![Line::from("Debug Bar")];
    let debug_bar = Paragraph::new(debug_text);

    frame.render_widget(debug_bar, area);
}
