use std::io::{stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use pages::dialog_page::DialogPage;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::pages::{chat_page::ChatPage, page::Page};
mod app;
mod chat;
mod events;
mod dialog_input;
mod utils;
mod pages;

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
    let mut terminal = init_terminal().expect("Failed to initialize terminal");

    let mut login_page = DialogPage::new(Some("Username".into()), None);
    let username = login_page.run(&mut terminal).await;

    let mut server_page = DialogPage::new(Some("IP Address".into()), Some("localhost:8080".into()));
    let address = server_page.run(&mut terminal).await;

    let mut chat_page = ChatPage::new(username, address);
    let success = chat_page.run(&mut terminal).await;

    match success {
        Err(_) => println!("Failed to connect to the server"),
        Ok(_) => ()
    }

    restore_terminal();

    println!("Bye!");
}
