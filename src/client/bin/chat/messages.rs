use crossterm::event::{self, Event, MouseEventKind};
use ratatui::{
    layout::Margin,
    text::Line,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

#[derive(Debug, Default)]
pub struct ChatMessages;

#[derive(Default, Debug, Clone)]
pub struct ChatMessagesState {
    pub messages: Vec<String>,
    pub vertical_scroll: usize,
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

        let foo: Vec<Line> = state
            .messages
            .iter()
            .map(|message| Line::from(message.to_string()))
            .collect();
        Paragraph::new(foo)
            .block(Block::new().borders(Borders::ALL))
            .scroll((state.vertical_scroll as u16, 0))
            .render(area, buf);
    }
}
