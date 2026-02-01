use std::fmt::Display;

use nix::sys::signal::Signal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Code(i32),
    Signal(Signal),
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitCode::Code(code) => write!(f, "{}", code),
            ExitCode::Signal(signal) => write!(f, "{}", signal),
        }
    }
}
