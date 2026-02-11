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
    event::{AppEvent, handle_key_event},
    model::{App, Command},
    process::ProcessManager,
    ui::{self, FrameContext, ViewportMetrics},
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

    app.add_command(Command::new("cargo fmt"));
    app.add_command(Command::new(
        "cargo clippy --fix --bin yagura --allow-dirty",
    ));
    app.add_command(Command::new("cargo test"));
    app.add_command(Command::new("cargo build"));
    app.add_command(Command::new("cargo build --release"));

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

    let cancel_tick_token = cancel_token.clone();
    let tick_tx = event_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
        loop {
            if cancel_tick_token.is_cancelled() {
                break;
            }
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
    let mut frame_context = FrameContext::default();
    loop {
        terminal.draw(|f| {
            frame_context = ui::build_frame_context(f);
            ui::render(f, app, &frame_context)
        })?;

        if let Some(event) = event_rx.recv().await {
            match event {
                AppEvent::Tick => {
                    // ignore
                }
                AppEvent::Key(key) => {
                    let viewport_metrics = ViewportMetrics::from(&frame_context);
                    handle_key_event(
                        app,
                        process_manager,
                        key,
                        event_tx.clone(),
                        viewport_metrics,
                    )
                    .await?
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
