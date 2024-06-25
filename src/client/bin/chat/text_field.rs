use ratatui::{prelude::*, widgets::*};

pub struct TextField;

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
        Paragraph::new(self.label.clone())
            .block(Block::new().title(self.placeholder.clone()).borders(Borders::ALL))
            .render(area, buf);
    }
}