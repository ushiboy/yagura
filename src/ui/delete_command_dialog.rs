use crate::model::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App) {
    let per_y = 20;
    let per_x = 60;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - per_y) / 2),
            Constraint::Percentage(per_y),
            Constraint::Percentage((100 - per_y) / 2),
        ])
        .split(frame.area());

    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - per_x) / 2),
            Constraint::Percentage(per_x),
            Constraint::Percentage((100 - per_x) / 2),
        ])
        .split(layout[1])[1];

    let block = Block::default()
        .title(" Confirm Delete ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);

    let command = app
        .get_selected_command()
        .map(|c| c.command())
        .unwrap_or("");

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Delete \"{}\"?", command),
            Style::default().fg(Color::Red),
        )),
        Line::from(""),
        Line::from("Press 'y' to confirm, 'n' to cancel"),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, inner);
}
