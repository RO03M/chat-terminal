use crossterm::event;
use ratatui::widgets::Widget;

use crate::{dialog_input::DialogInput, events::EventHandler};

use super::page::Page;

pub struct DialogPage {
    pub dialog: DialogInput,
    running: bool
}

impl Page for DialogPage {
    type RunResult = String;
    async fn run(&mut self, terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) -> Self::RunResult {
        while self.running {
            terminal
                .draw(|frame| {
                    frame.render_widget(&*self, frame.size());
                })
                .expect("Failed to render");

            self.handle_events().await;
        }

        return self.dialog.textfield_widget.value.clone();
    }

    async fn handle_events(&mut self) {
        let event = event::read().unwrap();
        self.dialog.on_event(event.clone());

        match event {
            event::Event::Key(key) => {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Enter => self.running = false,
                        event::KeyCode::Esc => self.running = false,
                        _ => ()
                    }
                }
            },
            _ => ()
        }
    }
}

impl DialogPage {
    pub fn new(label: Option<String>, default_value: Option<String>) -> Self {
        let label = label.unwrap_or_else(|| "".into());
        let default_value = default_value.unwrap_or_else(|| "".into());
        
        Self {
            dialog: DialogInput::new(label, default_value),
            running: true
        }
    }
}

impl EventHandler for DialogPage {
    fn on_event(&mut self, _event: event::Event) {
        self.dialog.on_event(_event.clone());
    }
}

impl Widget for &DialogPage {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        self.dialog.render(area, buf);
    }
}