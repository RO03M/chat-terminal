use ratatui::{prelude::*, widgets::*};

use crate::events::EventHandler;

#[derive(Default, Debug, Clone)]
pub struct TextField {
    pub value: String,
    pub placeholder: String,
    pub label: String,
    focused: bool
}

impl TextField {
    pub fn clear(&mut self) {
        self.value = "".into();
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl EventHandler for TextField {
    fn on_event(&mut self, _event: crossterm::event::Event) {
        if !self.focused {
            return;
        }

        match _event {
            crossterm::event::Event::Key(key_event) => {
                if key_event.kind == crossterm::event::KeyEventKind::Press {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            self.value += &c.to_string();
                        }
                        crossterm::event::KeyCode::Backspace => {
                            let mut input_value = self.value.clone().to_string();
                            input_value.pop();
                            
                            self.value = input_value;
                        },
                        _ => ()
                    }
                }
            },
            _ => ()
        }
    }
}

impl Widget for &TextField {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        Paragraph::new(self.value.clone())
            .block(Block::new().title(self.label.clone()).borders(Borders::ALL))
            .render(area, buf);
    }
}