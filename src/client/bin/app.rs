use std::{
    io::{self, stdout, Stdout},
    net::TcpStream,
    time::Duration,
};

use crate::chat::ui::Chat;
use crossterm::{
    event::{
        self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
        KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures_util::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::Margin,
    text::Line,
    widgets::{
        block::Title, Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Widget,
    },
    Frame, Terminal,
};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug)]
pub struct App {
    // terminal: Tui,
    chat: Chat,
    counter: u8,
    running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            chat: Chat::default(),
            counter: 0,
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

    pub fn teste(self, teste: String) {
        println!("{teste}");
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
        // match event::poll(Duration::from_millis(100)) {
        //     Ok(..) => match event::read().unwrap() {
        //         Event::Key(key_event) => {
        //             if key_event.kind == KeyEventKind::Press {
        //                 self.on_key_press(key_event);
        //             }
        //         }
        //         Event::Mouse(mouse_event) => match mouse_event.kind {
        //             MouseEventKind::ScrollUp => {
        //                 self.chat.on_scroll_up();
        //             }
        //             MouseEventKind::ScrollDown => {
        //                 self.chat.on_scroll_down();
        //             }
        //             _ => {}
        //         },
        //         _ => {
        //             println!("t");
        //         }
        //     },
        //     _ => {}
        // }

        // Ok(())
    }

    fn on_key_press(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Right => self.increment(),
            KeyCode::Backspace => self.chat.add_message("Beleza le".into()),
            _ => {}
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn exit(&mut self) {
        println!("exiting");
        self.running = false;
    }

    fn init() -> Result<Tui, std::io::Error> {
        stdout().execute(EnterAlternateScreen).unwrap();
        stdout().execute(EnableMouseCapture).unwrap();
        enable_raw_mode().expect("Failed to enable raw mode");

        Terminal::new(CrosstermBackend::new(stdout()))

        // let mut stdout = io::stdout();
        // execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
        // let backend = CrosstermBackend::new(stdout);
        // Terminal::new(backend)
    }

    fn restore(terminal: &mut Tui) {
        stdout().execute(LeaveAlternateScreen).unwrap();
        stdout().execute(DisableMouseCapture).unwrap();
        disable_raw_mode().expect("Failed to disable raw mode");

        // execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
        // terminal.show_cursor();
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(format!("counter: {}", self.counter)).render(area, buf);
    }
}
