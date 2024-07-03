use ratatui::{style::{Color, Style}, widgets::{Block, Widget}};

use crate::{chat::text_field::TextField, utils::centered_rect};

pub struct DialogInput {
    title: String,
    textfield_widget: TextField
}

impl DialogInput {

}

impl Widget for &DialogInput {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        let popup_block = Block::default()
        .title("Enter a new key-value pair")
        // .borders(Borders::NONE)
        .style(Style::default().bg(Color::Blue));
    
        let area = centered_rect(70, 25, area);

        // frame.render_widget(Clear, area);
        // frame.render_widget(popup_block, area);
    }
}