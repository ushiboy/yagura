use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App) {
    let per_y = 35;
    let per_x = 70;

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
        .title(" Add Command ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(inner);

    let command_label = Paragraph::new("Enter command:").style(Style::default().fg(Color::White));
    let command_input =
        Paragraph::new(app.form().command_input()).style(Style::default().fg(Color::Gray));

    let working_dir_label =
        Paragraph::new("Working directory (optional):").style(Style::default().fg(Color::White));
    let working_dir_input =
        Paragraph::new(app.form().working_dir_input()).style(Style::default().fg(Color::Gray));

    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Tab: switch fields | Enter: add | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    let help = Paragraph::new(help_text).alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(command_label, chunks[0]);
    frame.render_widget(command_input, chunks[1]);
    frame.render_widget(working_dir_label, chunks[3]);
    frame.render_widget(working_dir_input, chunks[4]);
    frame.render_widget(help, chunks[4]);
}
