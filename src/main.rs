use anyhow::Result;
use crossterm::{
    event::Event,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{io, time::Duration};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use yagura::{
    app::{App, add_command},
    event::{AppEvent, handle_key_event},
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

    add_command(&mut app, "date".to_string(), None);
    add_command(&mut app, "ls -alt".to_string(), None);
    add_command(&mut app, "echo 'Hello World!'".to_string(), None);
    add_command(&mut app, "sh ./tmp/ping.sh".to_string(), None);

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
                AppEvent::Key(key) => {
                    handle_key_event(app, process_manager, key, event_tx.clone()).await?
                }
                AppEvent::ProcessOutput(command_id, output_line) => {
                    app.add_output_line(command_id, output_line);
                }
                AppEvent::ProcessExited(command_id, exit_code) => {
                    app.mark_command_exit(command_id, exit_code);
                    process_manager.on_process_existed(command_id);
                }
            }
        }

        if app.should_quit() {
            break;
        }
    }

    Ok(())
}
