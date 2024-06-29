use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct TextField;

#[derive(Default, Clone, Debug)]
pub struct TextFieldState {
    pub value: String,
    pub placeholder: String,
    pub label: String
}

impl StatefulWidget for &TextField {
    type State = TextFieldState;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
        where
            Self: Sized {
        Paragraph::new(state.value.clone())
            .block(Block::new().title(state.label.clone()).borders(Borders::ALL))
            .render(area, buf);
    }
}