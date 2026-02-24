use anyhow::Result;
use crossterm::{
    event::Event,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::path::PathBuf;
use std::{io, time::Duration};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use yagura::{
    config,
    event::{AppEvent, handle_key_event},
    model::{App, Command},
    process::ProcessManager,
    ui::{self, FrameContext, ViewportMetrics},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = ".yagura.yaml")]
    config_path: Option<PathBuf>,
}

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> Result<Self> {
        enable_raw_mode()?;

        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let _terminal_guard = TerminalGuard::new()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut process_manager = ProcessManager::new();

    if let Some(config_path) = args.config_path
        && config_path.exists()
    {
        let conf = config::load_config(config_path)?;

        for cmd_conf in conf.commands {
            if cmd_conf.working_dir.is_none() {
                app.add_command(Command::new(cmd_conf.command));
            } else {
                app.add_command(
                    Command::new(cmd_conf.command).with_working_dir(cmd_conf.working_dir),
                );
            }
        }
    }

    let cancel_token = CancellationToken::new();
    let (event_tx, mut event_rx) = mpsc::channel::<AppEvent>(2048);

    let cancel_terminal_token = cancel_token.clone();
    let terminal_event_tx = event_tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel_terminal_token.cancelled() => {
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    if crossterm::event::poll(Duration::from_millis(0)).unwrap_or(false)
                        && let Ok(Event::Key(event)) = crossterm::event::read() {
                            let _ = terminal_event_tx.send(AppEvent::Key(event)).await;
                        }
                }
            }
        }
    });

    let cancel_tick_token = cancel_token.clone();
    let tick_tx = event_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(16));
        loop {
            if cancel_tick_token.is_cancelled() {
                break;
            }
            interval.tick().await;
            let _ = tick_tx.send(AppEvent::Tick).await;
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
    terminal.show_cursor()?;

    Ok(())
}

async fn main_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    process_manager: &mut ProcessManager,
    event_rx: &mut mpsc::Receiver<AppEvent>,
    event_tx: mpsc::Sender<AppEvent>,
) -> Result<()> {
    let mut frame_context = FrameContext::default();
    loop {
        if let Some(event) = event_rx.recv().await {
            match event {
                AppEvent::Tick => {
                    terminal.draw(|f| {
                        frame_context = ui::build_frame_context(f, app);
                        ui::render(f, app, &frame_context)
                    })?;
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
                    process_manager.on_process_exited(command_id);
                }
            }
        }

        if app.should_quit() {
            break;
        }
    }

    Ok(())
}
