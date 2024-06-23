use std::{
    io::{stdout, Stdout},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

use crate::app::App;
mod app;
mod chat;

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, std::io::Error> {
    stdout().execute(EnterAlternateScreen).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();
    enable_raw_mode().expect("Failed to enable raw mode");

    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    stdout().execute(DisableMouseCapture).unwrap();
    disable_raw_mode().expect("Failed to disable raw mode");
}

#[tokio::main]
async fn main() {
    let terminal = init_terminal().expect("Failed to initialize terminal");
    let mut app = App::default();

    app.run(terminal).await;

    restore_terminal();

    println!("Bye!");
}
