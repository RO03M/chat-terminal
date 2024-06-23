use crossterm::event;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::events::EventHandler;

use super::messages::{ChatMessages, ChatMessagesState};

#[derive(Debug)]
#[derive(Default)]
pub struct Chat {
    pub message_state: ChatMessagesState,
}

impl Chat {
    pub fn on_scroll_up(&mut self) {
        if self.message_state.vertical_scroll > 0 {
            self.message_state.vertical_scroll -= 1;
        }
    }

    pub fn on_scroll_down(&mut self) {
        self.message_state.vertical_scroll += 1;
    }
}

impl EventHandler for Chat {
    fn on_scroll(&mut self, _: event::MouseEvent, kind: event::MouseEventKind) {
        match kind {
            event::MouseEventKind::ScrollUp => self.on_scroll_up(),
            event::MouseEventKind::ScrollDown => self.on_scroll_up(),
            _ => ()
        }
    }
}

impl Widget for &Chat {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Fill(1)]);
        let [bar, content] = layout.areas(area);

        Paragraph::new("sidebar")
            .block(Block::new().borders(Borders::ALL))
            .render(bar, buf);

        ChatMessages.render(content, buf, &mut self.message_state.clone());
    }
}
