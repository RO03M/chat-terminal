
use ratatui::{
    layout::{Constraint, Layout, Margin},
    text::Line,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

use super::text_field::{TextField, TextFieldState};

#[derive(Debug, Default)]
pub struct ChatMessages;

#[derive(Default, Debug, Clone)]
pub struct ChatMessagesState {
    pub messages: Vec<String>,
    pub vertical_scroll: usize,
    pub textfield_state: TextFieldState
}

impl ChatMessagesState {
    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }
}

impl StatefulWidget for ChatMessages {
    type State = ChatMessagesState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state =
            ScrollbarState::new(state.messages.len()).position(state.vertical_scroll);

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

        let messages: Vec<Line> = state
            .messages
            .iter()
            .map(|message| Line::from(message.to_string()))
            .collect();

        Paragraph::new(messages)
            .block(Block::new().borders(Borders::ALL))
            .scroll((state.vertical_scroll as u16, 0))
            .render(list_area, buf);

        // let mut textarea = TextArea::default();
        // textarea.set_block(
        //     Block::default()
        //         .borders(Borders::ALL)
        //         .title("Test")
        // );

        // textarea.widget().render(input_area, buf);

        TextField::default().render(input_area, buf, &mut state.textfield_state);
    }
}
