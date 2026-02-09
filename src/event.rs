use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::{model::OutputLine, process::ExitCode};

pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    ProcessOutput(Uuid, OutputLine),
    ProcessExited(Uuid, ExitCode),
}

mod handle_adding_command_mode;
mod handle_deleting_command_mode;
mod handle_key_event;
mod handle_normal_mode;

pub use handle_key_event::handle_key_event;
