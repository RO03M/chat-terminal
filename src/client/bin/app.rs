use std::{
    io::{self, Stdout},
    time::Duration,
};

use crate::{chat::chat::Chat, events::EventHandler};
use crossterm::event::{self, KeyCode, KeyEvent};
use futures_util::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    Frame, Terminal,
};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

#[derive(Debug)]
pub struct App {
    chat: Chat,
    running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            chat: Chat::default(),
            running: true,
        }
    }
}

impl App {
    pub async fn run(&mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) {
        let (ws_stream, _response) = connect_async("ws://localhost:8080/chat")
            .await
            .expect("Failed to connect");

        let (_write, mut read) = ws_stream.split();

        while self.running {
            terminal
                .draw(|frame| {
                    self.handle_render(frame);
                })
                .expect("Failed to render");
            tokio::select! {
                received = read.next() => {
                    let received = received.unwrap();
                    let message = received.unwrap();
                    match message {
                        Message::Text(text) => {
                            self.chat.message_state.add_message(text);
                        },
                        _ => {}
                    }
                }
                _ = self.handle_events() => {}
            }
        }
    }

    fn handle_render(&self, frame: &mut Frame) {
        frame.render_widget(&self.chat, frame.size());
    }

    async fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(key_event) => {
                    if key_event.kind == event::KeyEventKind::Press {
                        self.on_key_press(key_event);
                    }
                }
                event::Event::Mouse(mouse_event) => {
                    self.chat.on_scroll(mouse_event, mouse_event.kind);
                }
                _ => ()
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_key_press(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc {
            self.exit();
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }
}
