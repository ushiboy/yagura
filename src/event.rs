use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::app::OutputLine;

pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    ProcessOutput(Uuid, OutputLine),
}
