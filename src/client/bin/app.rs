use std::{io::{self, stdout, Stdout}, net::TcpStream, time::Duration};

use crossterm::{event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, MouseEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, layout::Margin, text::Line, widgets::{block::Title, Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Widget}, Frame, Terminal};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};

use crate::chat::ui::Chat;

type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug)]
pub struct App {
    // terminal: Tui,
    chat: Chat,
    counter: u8,
    running: bool,
    socket: WebSocket<MaybeTlsStream<TcpStream>>
}

impl Default for App {
    fn default() -> Self {
        let (socket, response) = connect("ws://localhost:8080/chat").expect("Failed to connect to server");
        println!("{}", response.status());
        Self {
            // terminal: App::init().unwrap(),
            chat: Chat::default(),
            counter: 0,
            running: true,
            socket
        }
    }
}

impl App {
    pub fn run(&mut self) {
        let mut terminal = App::init().unwrap();

        while self.running {
            terminal.draw(|frame| {
                self.handle_render(frame);
            }).expect("Failed to render");

            self.handle_events();
            // self.handle_socket();
        }

        App::restore(&mut terminal);
    }

    pub fn teste(self, teste: String) {
        println!("{teste}");
    }

    fn handle_socket(&mut self) {
        let message = self.socket.read().expect("Error reading message");
        println!("{message}");
        // self.chat.add_message(format!("{message}"));
    }

    fn handle_render(&self, frame: &mut Frame) {
        frame.render_widget(&self.chat, frame.size());
    }

    fn handle_events(&mut self) -> Result<(), ()> {
        if event::poll(Duration::from_millis(100)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        self.on_key_press(key_event);
                    }

                    return Ok(());
                },
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => {
                            self.chat.on_scroll_up();

                            return Ok(());
                        },
                        MouseEventKind::ScrollDown => {
                            self.chat.on_scroll_down();

                            return Ok(());
                        },
                        _ => return Ok(())
                    }
                },
                _ => {
                    println!("t");
                    return Ok(());
                }
            }
        }

        Ok(())
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
            Self: Sized {
        Paragraph::new(format!("counter: {}", self.counter)).render(area, buf);
    }
}