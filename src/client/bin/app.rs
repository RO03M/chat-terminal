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
pub struct App {
    chat: Chat,
    running: bool,
    message_queue: Vec<String>
}

impl Default for App {
    fn default() -> Self {
        Self {
            chat: Chat::default(),
            running: true,
            message_queue: vec!["from queue!".into()]
        }
    }
}

impl App {
    pub async fn run(&mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) {
        let (ws_stream, _response) = connect_async("ws://localhost:8080/chat")
            .await
            .expect("Failed to connect");

        let (mut write, mut read) = ws_stream.split();

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
                _ = self.handle_events() => {
                    self.handle_queue(&mut write).await;
                }
            }
        }
    }

    fn handle_render(&self, frame: &mut Frame) {
        // frame.render_widget(&self.chat, frame.size());
        self.chat.ui(frame);
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
        match key_event.code {
            KeyCode::Esc => {
                self.exit();
            }
            KeyCode::Char(c) => {
                self.chat.message_state.textfield_state.value += &c.to_string();
            }
            KeyCode::Enter => {
                self.message_queue.push(self.chat.message_state.textfield_state.value.clone());
                self.chat.message_state.textfield_state.value = "".into();
            }
            KeyCode::Backspace => {
                let mut input_value = self.chat.message_state.textfield_state.value.clone().to_string();
                input_value.pop();
                
                self.chat.message_state.textfield_state.value = input_value;
            }
            _ => {}
        }
        // self.chat.message_state.textfield_state.value += key_event.state
    }

    fn exit(&mut self) {
        self.running = false;
    }
}
