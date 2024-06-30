
use ratatui::{
    layout::{Constraint, Layout, Margin},
    text::Line,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

use crate::events::EventHandler;

use super::text_field::TextField;

#[derive(Debug, Clone)]
pub struct ChatMessages {
    pub textfield_widget: TextField,
    pub messages: Vec<String>,
    pub vertical_scroll: usize
}

impl Default for ChatMessages {
    fn default() -> Self {
        let mut default = Self {
            messages: vec![],
            vertical_scroll: 0,
            textfield_widget: TextField::default()
        };

        default.textfield_widget.focus();

        default
    }
}

impl ChatMessages {
    pub fn on_scroll_up(&mut self) {
        if self.vertical_scroll > 0 {
            self.vertical_scroll -= 1;
        }
    }

    pub fn on_scroll_down(&mut self) {
        self.vertical_scroll += 1;
    }
}

impl EventHandler for ChatMessages {
    fn on_event(&mut self, event: crossterm::event::Event) {
        self.textfield_widget.on_event(event.clone());
        match event {
            crossterm::event::Event::Mouse(mouse_event) => {
                match mouse_event.kind {
                    crossterm::event::MouseEventKind::ScrollUp => self.on_scroll_up(),
                    crossterm::event::MouseEventKind::ScrollDown => self.on_scroll_down(),
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

impl Widget for &ChatMessages {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state =
            ScrollbarState::new(self.messages.len()).position(self.vertical_scroll);

        scrollbar.render(
            area.inner(&Margin {
                horizontal: 1,
                vertical: 0,
            }),
            buf,
            &mut scrollbar_state,
        );

        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Percentage(10)]);
        let [list_area, input_area] = layout.areas(area);

        let messages: Vec<Line> = self
            .messages
            .iter()
            .map(|message| Line::from(message.to_string()))
            .collect();

        Paragraph::new(messages)
            .block(Block::new().borders(Borders::ALL))
            .scroll((self.vertical_scroll as u16, 0))
            .render(list_area, buf);

        self.textfield_widget.render(input_area, buf);
    }
}
