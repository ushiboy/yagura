use crate::model::App;
use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: Rect, _app: &App) {
    let help_text = vec![Line::from(Span::styled(
        "[Ctrl + f] Page Down  [Ctrl + b] Page Up  [t] Timestamp Show/Hide",
        Style::default().fg(Color::White),
    ))];
    let help = Paragraph::new(help_text).alignment(Alignment::Center);

    frame.render_widget(help, area);
}
