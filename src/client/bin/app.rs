use std::{
    io::{self, stdout, Stdout},
    time::Duration,
};

use crate::chat::ui::Chat;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
        KeyEventKind, MouseEventKind,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures_util::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    Frame, Terminal,
};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

type Tui = Terminal<CrosstermBackend<Stdout>>;

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
    pub async fn run(&mut self) {
        let mut terminal = App::init().unwrap();

        let (ws_stream, response) = connect_async("ws://localhost:8080/chat")
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
                    // println!("{:?}", received.unwrap());
                    let received = received.unwrap();
                    let message = received.unwrap();
                    match message {
                        Message::Text(text) => {
                            self.chat.add_message(text);
                        },
                        _ => {}
                    }
                }
                _ = self.handle_events() => {}
            }
        }

        App::restore(&mut terminal);
    }

    fn handle_render(&self, frame: &mut Frame) {
        frame.render_widget(&self.chat, frame.size());
    }

    async fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        self.on_key_press(key_event);
                    }
                }
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        self.chat.on_scroll_up();
                    }
                    MouseEventKind::ScrollDown => {
                        self.chat.on_scroll_down();
                    }
                    _ => {}
                },
                _ => {}
            }

            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn on_key_press(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }

    fn init() -> Result<Tui, std::io::Error> {
        stdout().execute(EnterAlternateScreen).unwrap();
        stdout().execute(EnableMouseCapture).unwrap();
        enable_raw_mode().expect("Failed to enable raw mode");

        Terminal::new(CrosstermBackend::new(stdout()))
    }

    fn restore(terminal: &mut Tui) {
        stdout().execute(LeaveAlternateScreen).unwrap();
        stdout().execute(DisableMouseCapture).unwrap();
        disable_raw_mode().expect("Failed to disable raw mode");
    }
}
