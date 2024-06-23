use std::collections::HashSet;

use crossterm::event;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use super::messages::{ChatMessages, ChatMessagesState};

#[derive(Debug)]
#[derive(Default)]
pub struct Chat {
    messages: HashSet<String>,
    chat_message_state: ChatMessagesState,
}



impl Chat {
    pub fn handle_events(&self, _event: event::Event) {

    }

    pub fn on_scroll_up(&mut self) {
        if self.chat_message_state.vertical_scroll > 0 {
            self.chat_message_state.vertical_scroll -= 1;
        }
    }

    pub fn on_scroll_down(&mut self) {
        self.chat_message_state.vertical_scroll += 1;
    }

    pub fn add_message(&mut self, message: String) {
        self.chat_message_state.messages.push(message);
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

        ChatMessages.render(content, buf, &mut self.chat_message_state.clone());
    }
}
