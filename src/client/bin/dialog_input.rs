use ratatui::{style::{Color, Style}, widgets::{Block, Borders, Clear, Widget}};

use crate::{chat::text_field::TextField, events::EventHandler, utils::centered_rect};

pub struct DialogInput {
    title: String,
    pub textfield_widget: TextField
}

impl Default for DialogInput {
    fn default() -> Self {
        let mut textfield_widget = TextField::default();
        textfield_widget.focus();
        Self {
            title: "".into(),
            textfield_widget
        }
    }
}

impl EventHandler for DialogInput {
    fn on_event(&mut self, _event: crossterm::event::Event) {
        self.textfield_widget.on_event(_event);
    }
}

impl Widget for &DialogInput {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        // let popup_block = Block::default()
        // .title("Enter a new key-value pair")
        // .borders(Borders::all());
        // .style(Style::default().bg(Color::Blue));
        

        let centered_area = centered_rect(70, 25, area);

        Clear.render(centered_area, buf);
        self.textfield_widget.render(centered_area, buf);
    }
}