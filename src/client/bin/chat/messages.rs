
use ratatui::{
    layout::Margin,
    text::Line,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

use crate::events::EventHandler;

#[derive(Debug, Clone)]
pub struct ChatMessages {
    pub messages: Vec<String>,
    pub vertical_scroll: usize
}

impl Default for ChatMessages {
    fn default() -> Self {
        Self {
            messages: vec![],
            vertical_scroll: 0,
        }
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

        let messages: Vec<Line> = self
            .messages
            .iter()
            .map(|message| Line::from(message.to_string()))
            .collect();

        Paragraph::new(messages)
            .block(Block::new().borders(Borders::ALL))
            .scroll((self.vertical_scroll as u16, 0))
            .render(area, buf);
    }
}
