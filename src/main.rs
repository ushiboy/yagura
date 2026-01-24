use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{io, time::Duration};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use yagura::{
    app::{App, add_command},
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

    add_command(&mut app, "date".to_string());
    add_command(&mut app, "ls -alt".to_string());
    add_command(&mut app, "echo 'Hello World!'".to_string());

    let cancel_token = CancellationToken::new();
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<AppEvent>();

    let cancel_terminal_token = cancel_token.clone();
    let terminal_event_tx = event_tx.clone();
    tokio::spawn(async move {
        loop {
            if cancel_terminal_token.is_cancelled() {
                break;
            }

            if crossterm::event::poll(Duration::from_millis(100)).is_ok()
                && let Ok(Event::Key(key)) = crossterm::event::read()
            {
                let _ = terminal_event_tx.send(AppEvent::Key(key));
            }

            tokio::task::yield_now().await;
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

    cancel_token.cancel();
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
                    KeyCode::Char('j') | KeyCode::Down => app.select_next_commmand(),
                    KeyCode::Char('k') | KeyCode::Up => app.select_previous_command(),
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
