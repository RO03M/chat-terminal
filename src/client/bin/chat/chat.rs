use crossterm::event;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Widget}, Frame,
};

use crate::events::EventHandler;

use super::messages::ChatMessages;

#[derive(Default, Debug, Clone)]
pub struct Chat {
    pub messages_widget: ChatMessages
}

impl Chat {
    pub fn ui(&self, frame: &mut Frame) {
        let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Fill(1)]);
        let [bar, content] = layout.areas(frame.size());

        frame.render_widget(
            Paragraph::new("sidebar")
                .block(Block::new().borders(Borders::ALL)),
            bar
        );

        self.messages_widget.clone().render(content, frame.buffer_mut());
    }
}

impl EventHandler for Chat {
    fn on_event(&mut self, _event: event::Event) {
        self.messages_widget.on_event(_event.clone());
    }
}
