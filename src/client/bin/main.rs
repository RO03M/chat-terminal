use std::{io::{stdout, Stdout}, thread, time::Duration};

use crossterm::{event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};
use tungstenite::{connect, Message};

use crate::app::App;
mod app;
mod chat;

fn main() {
    // let (mut socket, response) = connect("ws://localhost:8080/chat").expect("Failed to connect to server");

    // println!("Response http code: {}", response.status());

    // socket.send(Message::Text("Beleza LÊ".into())).unwrap();

    // loop {
    //     let message = socket.read().expect("Error reading message");

    //     println!("{message}");
    // }
    // let mut terminal = init().expect("Failed to init terminal");
    // terminal.clear().expect("Failed to clear terminal");
    // loop {
    //     let _ = terminal.draw(|frame| {
    //         let area = frame.size();
    
    //         frame.render_widget(
    //             Paragraph::new("Beleza Lê")
    //                 .white()
    //                 .on_blue(),
    //                 area
    //             );
    //     }).unwrap();

    //     if event::poll(Duration::from_millis(16)).unwrap() {
    //         if let event::Event::Key(key) = event::read().unwrap() {
    //             if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
    //                 break;
    //             }
    //         }
    //     }
    // // }
    let mut app = App::default();

    app.run();
    
    println!("Bye!");
}