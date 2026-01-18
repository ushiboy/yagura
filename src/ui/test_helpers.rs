#[cfg(test)]
use ratatui::Terminal;
#[cfg(test)]
use ratatui::backend::TestBackend;

#[cfg(test)]
/// Helper function to setup a test terminal
pub fn setup_test_terminal(width: u16, height: u16) -> Terminal<TestBackend> {
    let backend = TestBackend::new(width, height);
    Terminal::new(backend).unwrap()
}
