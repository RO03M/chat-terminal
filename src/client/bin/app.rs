use std::{
    io::{self, Stdout},
    time::Duration, vec,
};

use crate::{chat::chat::Chat, events::EventHandler};
use crossterm::event::{self, KeyCode, KeyEvent};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use ratatui::{
    backend::CrosstermBackend,
    Frame, Terminal,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

#[derive(Debug)]
pub enum AppModes {
    EDITING,
    NORMAL
}

#[derive(Debug)]
pub struct App {
    chat: Chat,
    running: bool,
    message_queue: Vec<String>,
    address: String,
    mode: AppModes
}

impl Default for App {
    fn default() -> Self {
        Self {
            chat: Chat::default(),
            running: true,
            message_queue: vec!["from queue!".into()],
            address: "localhost:8080".into(),
            mode: AppModes::NORMAL
        }
    }
}

impl App {
    pub async fn run(&mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) {
        let (ws_stream, _response) = connect_async(format!("ws://{}/chat", self.address))
            .await
            .expect("Failed to connect");

        let (mut write, mut read) = ws_stream.split();

        while self.running {
            terminal
                .draw(|frame| {
                    self.update(frame);
                })
                .expect("Failed to render");

            tokio::select! {
                received = read.next() => {
                    let received = received.unwrap();
                    let message = received.unwrap();
                    match message {
                        Message::Text(text) => {
                            self.chat.messages_widget.messages.push(text);
                        },
                        _ => {}
                    }
                }
                _ = self.handle_events() => {
                    self.handle_queue(&mut write).await;
                }
            }
        }
    }

    fn update(&mut self, frame: &mut Frame) {
        self.chat.ui(frame);

        match self.mode {
            AppModes::EDITING => self.chat.messages_widget.textfield_widget.focus(),
            AppModes::NORMAL => self.chat.messages_widget.textfield_widget.unfocus()
        };
    }

    async fn handle_queue(&mut self, write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
        let first_message = self.message_queue.pop();

        match first_message {
            Some(message) => {
                let _ = write.send(Message::Text(message)).await;
            }
            None => {}
        }
    }

    async fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            let event = event::read().unwrap();
            self.chat.on_event(event.clone());
            match event {
                event::Event::Key(key_event) => {
                    if key_event.kind == event::KeyEventKind::Press {
                        self.on_key_press(key_event);
                    }
                }
                _ => ()
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_key_press(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                match self.mode {
                    AppModes::EDITING => {
                        self.mode = AppModes::NORMAL;
                    }
                    AppModes::NORMAL => {
                        self.exit();
                    }
                }
            }
            KeyCode::Char('e') => {
                match self.mode {
                    AppModes::NORMAL => {
                        self.mode = AppModes::EDITING;
                    }
                    _ => ()
                }
            }
            KeyCode::Enter => {
                self.message_queue.push(self.chat.messages_widget.textfield_widget.value.clone());
                self.chat.messages_widget.textfield_widget.clear();
            }
            _ => {}
        }

    }

    fn exit(&mut self) {
        self.running = false;
    }
}
