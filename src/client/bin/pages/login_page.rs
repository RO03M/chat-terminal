use crossterm::event;
use ratatui::widgets::Widget;

use crate::{dialog_input::DialogInput, events::EventHandler};

pub struct LoginPage {
    pub dialog: DialogInput
}

impl Default for LoginPage {
    fn default() -> Self {
        Self {
            dialog: DialogInput::default()
        }
    }
}

impl EventHandler for LoginPage {
    fn on_event(&mut self, _event: event::Event) {
        self.dialog.on_event(_event.clone());
    }
}

impl Widget for &LoginPage {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        self.dialog.render(area, buf);
    }
}