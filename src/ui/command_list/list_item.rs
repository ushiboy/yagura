use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::app::Command;

pub fn list_item(cmd: &Command, is_selected: bool) -> ListItem<'_> {
    let marker = if is_selected { "[*]" } else { "[ ]" };
    let content = format!("{} {}", marker, cmd.command());

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    ListItem::new(Line::from(Span::styled(content, style)))
}
