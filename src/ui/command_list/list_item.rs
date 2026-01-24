use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::{app::Command, ui::command_list::display_status::display_status};

pub fn list_item(cmd: &Command, is_selected: bool) -> ListItem<'_> {
    let marker = if is_selected { "[*]" } else { "[ ]" };
    let status = display_status(cmd.status());
    let content = format!("{} {} {}", marker, cmd.command(), status);

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    ListItem::new(Line::from(Span::styled(content, style)))
}
