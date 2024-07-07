use std::io::{stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::pages::{login_page::LoginPage, server_page::ServerPage, chat_page::ChatPage, page::Page};
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

    let mut login_page = LoginPage::default();
    let username = login_page.run(&mut terminal).await;

    let mut server_page = ServerPage::default();
    let address = server_page.run(&mut terminal).await;

    let mut chat_page = ChatPage::new(username, address);
    let success = chat_page.run(&mut terminal).await;

    match success {
        Err(_) => println!("Failed to connect to the server"),
        Ok(_) => ()
    }

    // let mut app = App::new(username, address);

    // app.run(terminal).await;

    restore_terminal();

    println!("Bye!");
}
