use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::{app::OutputLine, process::ExitCode};

pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    ProcessOutput(Uuid, OutputLine),
    ProcessExited(Uuid, ExitCode),
}

mod handle_normal_mode;
pub use handle_normal_mode::handle_normal_mode;
