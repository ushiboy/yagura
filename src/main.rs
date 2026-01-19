use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;
use tokio::sync::mpsc;
use yagura::{
    app::{App, Command},
    event::AppEvent,
    process::ProcessManager,
    ui,
};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut process_manager = ProcessManager::new();

    let command = Command::new("echo 'hello'".to_string());
    let command_id = command.id();
    app.add_command(command);
    app.select_command_by_id(command_id);

    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<AppEvent>();

    let terminal_event_tx = event_tx.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(Event::Key(key)) = crossterm::event::read() {
                let _ = terminal_event_tx.send(AppEvent::Key(key));
            }
        }
    });

    let tick_tx = event_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            let _ = tick_tx.send(AppEvent::Tick);
        }
    });

    main_loop(
        &mut terminal,
        &mut app,
        &mut process_manager,
        &mut event_rx,
        event_tx,
    )
    .await?;

    process_manager.shutdown_all().await?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

async fn main_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    process_manager: &mut ProcessManager,
    event_rx: &mut mpsc::UnboundedReceiver<AppEvent>,
    event_tx: mpsc::UnboundedSender<AppEvent>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Some(event) = event_rx.recv().await {
            match event {
                AppEvent::Tick => {
                    // ignore
                }
                AppEvent::Key(key) => match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit()
                    }
                    KeyCode::Enter => {
                        if let Some(command) = app.get_selected_command() {
                            process_manager.spawn(command, event_tx.clone()).await?;
                        }
                    }
                    _ => {}
                },
                AppEvent::ProcessOutput(command_id, output_line) => {
                    app.add_output_line(command_id, output_line);
                }
            }
        }

        if app.should_quit() {
            break;
        }
    }

    Ok(())
}
