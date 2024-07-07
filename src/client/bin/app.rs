// use std::{
//     any::Any, collections::HashMap, io::{self, Stdout}, time::Duration, vec
// };

// use crate::{chat::chat::Chat, dialog_input::DialogInput, events::EventHandler, pages::{login_page::LoginPage, server_page::ServerPage}};
// use crossterm::event::{self, KeyCode, KeyEvent};
// use futures_util::{stream::SplitSink, SinkExt, StreamExt};
// use ratatui::{prelude::*, widgets::{Block, Borders, Clear}};
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use tokio::net::TcpStream;
// use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
// use tungstenite::Message;

// #[derive(Debug)]
// pub enum AppState {
//     Editing,
//     Normal
// }

// #[derive(Debug, PartialEq)]
// pub enum CurrentPage {
//     Login,
//     ServerAddress,
//     Chat
// }

// #[derive(Debug, Default)]
// pub struct UserData {
//     name: String
// }

// #[derive(Serialize, Deserialize)]
// pub struct ServerMessage {
//     author: String,
//     message: String
// }

// pub struct App {
//     chat: Chat,
//     login_page: LoginPage,
//     server_page: ServerPage,
//     running: bool,
//     message_queue: Vec<String>,
//     address: String,
//     app_state: AppState,
//     user_data: UserData,
//     current_page: CurrentPage
// }

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             chat: Chat::default(),
//             login_page: LoginPage::default(),
//             server_page: ServerPage::default(),
//             running: true,
//             message_queue: vec!["from queue!".into()],
//             address: "localhost:8080".into(),
//             app_state: AppState::Normal,
//             current_page: CurrentPage::Chat,
//             user_data: UserData {
//                name: "romera".into()
//             }
//         }
//     }
// }

// impl App {
//     pub fn new(username: String, address: String) -> App {
//         App {
//             chat: Chat::default(),
//             login_page: LoginPage::default(),
//             server_page: ServerPage::default(),
//             running: true,
//             message_queue: vec!["from queue!".into()],
//             address,
//             app_state: AppState::Normal,
//             current_page: CurrentPage::Chat,
//             user_data: UserData {
//                name: username
//             }
//         }
//     }

//     pub async fn run(&mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) {
//         let (ws_stream, _response) = connect_async(format!("ws://{}/chat", self.address))
//             .await
//             .expect("Failed to connect");

//         let (mut write, mut read) = ws_stream.split();

//         while self.running {
//             terminal
//                 .draw(|frame| {
//                     self.update(frame);
//                 })
//                 .expect("Failed to render");

//             tokio::select! {
//                 received = read.next() => {
//                     let received = received.unwrap();
//                     let message = received.unwrap();
//                     match message {
//                         Message::Text(message) => {
//                             let message: Result<ServerMessage, serde_json::Error> = serde_json::from_str(&message);
//                             match message {
//                                 Ok(decoded_message) => {
//                                     self.chat.messages_widget.messages.push(format!("<{}> {}", decoded_message.author, decoded_message.message));
//                                 },
//                                 Err(_) => {
//                                     self.chat.messages_widget.messages.push("Failed to receive message, this is a fallback".into());
//                                 },
//                             }
//                         },
//                         _ => {}
//                     }
//                 }
//                 _ = self.handle_events() => {
//                     self.handle_queue(&mut write).await;
//                 }
//             }
//         }
//     }

//     fn update(&mut self, frame: &mut Frame) {
//         match self.current_page {
//             CurrentPage::Login => {
//                 frame.render_widget(&self.login_page, frame.size());
//             },
//             CurrentPage::ServerAddress => {
//                 frame.render_widget(&self.login_page, frame.size());
//             },
//             CurrentPage::Chat => {
//                 frame.render_widget(&self.chat, frame.size());
                
//                 match self.app_state {
//                     AppState::Editing => self.chat.messages_widget.textfield_widget.focus(),
//                     AppState::Normal => self.chat.messages_widget.textfield_widget.unfocus()
//                 };
//             },
//         }
//     }

//     async fn handle_queue(&mut self, write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
//         let first_message = self.message_queue.pop();

//         match first_message {
//             Some(message) => {
//                 let message = json!({
//                     "author": self.user_data.name,
//                     "message": message
//                 });
//                 let _ = write.send(Message::Text(message.to_string())).await;
//             }
//             None => {}
//         }
//     }

//     async fn handle_events(&mut self) -> io::Result<bool> {
//         if event::poll(Duration::from_millis(100))? {
//             let event = event::read().unwrap();
//             match self.current_page {
//                 CurrentPage::Login => self.login_page.on_event(event.clone()),
//                 CurrentPage::ServerAddress => self.server_page.on_event(event.clone()),
//                 CurrentPage::Chat => self.chat.on_event(event.clone()),
//             }
//             match event {
//                 event::Event::Key(key_event) => {
//                     if key_event.kind == event::KeyEventKind::Press {
//                         self.on_key_press(key_event);
//                     }
//                 }
//                 _ => ()
//             }

//             Ok(true)
//         } else {
//             Ok(false)
//         }
//     }

//     fn on_key_press(&mut self, key_event: KeyEvent) {
//         match key_event.code {
//             KeyCode::Esc => {
//                 match self.app_state {
//                     AppState::Editing => {
//                         self.app_state = AppState::Normal;
//                     }
//                     AppState::Normal => {
//                         self.exit();
//                     }
//                 }
//             }
//             KeyCode::Char('e') => {
//                 match self.app_state {
//                     AppState::Normal => {
//                         self.app_state = AppState::Editing;
//                     }
//                     _ => ()
//                 }
//             }
//             KeyCode::Enter => {
//                 match self.current_page {
//                     CurrentPage::Login => {
//                         self.user_data.name = self.login_page.dialog.textfield_widget.value.clone();
//                         self.login_page.dialog.textfield_widget.clear();
//                         self.current_page = CurrentPage::ServerAddress;
//                     },
//                     CurrentPage::ServerAddress => {
//                         self.address = self.server_page.dialog.textfield_widget.value.clone();
//                         self.server_page.dialog.textfield_widget.clear();
//                         self.current_page = CurrentPage::Chat;
//                     },
//                     CurrentPage::Chat => {
//                         self.message_queue.push(self.chat.messages_widget.textfield_widget.value.clone());
//                         self.chat.messages_widget.textfield_widget.clear();
//                     },
//                 }
//             }
//             _ => {}
//         }

//     }

//     fn exit(&mut self) {
//         self.running = false;
//     }
// }
