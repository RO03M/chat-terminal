use std::{
    io::{stdout, Stdout},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

use crate::app::App;
mod app;
mod chat;

#[tokio::main]
async fn main() {
    let mut app = App::default();

    app.run().await;

    println!("Bye!");
}
