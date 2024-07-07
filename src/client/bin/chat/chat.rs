use crossterm::event;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Paragraph, Widget}
};

use crate::events::EventHandler;

use super::{messages::ChatMessages, text_field::TextField};

#[derive(Default, Debug, Clone)]
pub struct Chat {
    pub messages_widget: ChatMessages,
    pub textfield_widget: TextField,
}

impl EventHandler for Chat {
    fn on_event(&mut self, _event: event::Event) {
        self.messages_widget.on_event(_event.clone());
        self.textfield_widget.on_event(_event.clone());
    }
}

impl Widget for &Chat {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Percentage(10), Constraint::Percentage(15)]);
        let [list_area, input_area, footer_area] = layout.areas(area);

        self.messages_widget.clone().render(list_area, buf);
        self.textfield_widget.render(input_area, buf);

        let footer_message = if self.textfield_widget.is_focused() {
            "<esc> Leave edit mode, <enter> Submit message"
        } else {
            "<esc> Quit, <e> Enter edit mode"
        };

        Paragraph::new(footer_message)
            .render(footer_area, buf);
    }
}