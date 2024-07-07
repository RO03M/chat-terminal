use std::time::Duration;

use crossterm::event;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use ratatui::widgets::Widget;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use crate::{chat::chat::Chat, events::EventHandler};

use super::page::Page;

#[derive(Serialize, Deserialize)]
struct ServerMessage {
    author: String,
    message: String
}

#[derive(Debug)]
enum AppState {
    Editing,
    Normal
}

pub struct ChatPage {
    running: bool,
    address: String,
    username: String,
    chat: Chat,
    app_state: AppState,
    message_queue: Vec<String>,
}

impl ChatPage {
    pub fn new(username: String, address: String) -> ChatPage {
        ChatPage {
            running: true,
            address,
            username,
            app_state: AppState::Normal,
            chat: Chat::default(),
            message_queue: vec![]
        }
    }

    async fn handle_queue(&mut self, write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
        let first_message = self.message_queue.pop();

        match first_message {
            Some(message) => {
                let message = json!({
                    "author": self.username,
                    "message": message
                });
                let _ = write.send(Message::Text(message.to_string())).await;
            }
            None => {}
        }
    }
}

impl Page for ChatPage {
    type RunResult = ();
    async fn run(&mut self, terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) -> Self::RunResult {
        let (ws_stream, _response) = connect_async(format!("ws://{}/chat", self.address))
            .await
            .expect(&format!("Failed to connect to {}", self.address));

        let (mut write, mut read) = ws_stream.split();

        while self.running {
            terminal
                .draw(|frame| {
                    frame.render_widget(&self.chat, frame.size());
                
                    match self.app_state {
                        AppState::Editing => self.chat.messages_widget.textfield_widget.focus(),
                        AppState::Normal => self.chat.messages_widget.textfield_widget.unfocus()
                    };
                })
                .expect("Failed to render");

            tokio::select! {
                received = read.next() => {
                    let received = received.unwrap();
                    let message = received.unwrap();
                    match message {
                        Message::Text(message) => {
                            let message: Result<ServerMessage, serde_json::Error> = serde_json::from_str(&message);
                            match message {
                                Ok(decoded_message) => {
                                    self.chat.messages_widget.messages.push(format!("<{}> {}", decoded_message.author, decoded_message.message));
                                },
                                Err(_) => {
                                    self.chat.messages_widget.messages.push("Failed to receive message, this is a fallback".into());
                                },
                            }
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

    async fn handle_events(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            let event = event::read().unwrap();
            self.chat.on_event(event.clone());
    
            match event {
                event::Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            event::KeyCode::Enter => {
                                self.message_queue.push(self.chat.messages_widget.textfield_widget.value.clone());
                                self.chat.messages_widget.textfield_widget.clear();
                            }
                            event::KeyCode::Esc => {
                                match self.app_state {
                                    AppState::Editing => {
                                        self.app_state = AppState::Normal;
                                    }
                                    AppState::Normal => {
                                        self.running = false;
                                    }
                                }
                            }
                            event::KeyCode::Char('e') => {
                                match self.app_state {
                                    AppState::Normal => {
                                        self.app_state = AppState::Editing;
                                    }
                                    _ => ()
                                }
                            }
                            _ => ()
                        }
                    }
    
                },
                _ => ()
            }
        }
    }
}

impl EventHandler for ChatPage {
    fn on_event(&mut self, _event: event::Event) {
        self.chat.on_event(_event.clone());
    }
}

impl Widget for &ChatPage {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        self.chat.render(area, buf);
    }
}