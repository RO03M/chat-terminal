use ratatui::{prelude::*, widgets::*};

#[derive(Debug)]
pub struct UI {
    // messages: Vec<String>
    pub vertical_scroll: usize
}

impl Default for UI {
    fn default() -> Self {
        Self {
            vertical_scroll: 0
        }
    }
}

impl Widget for &UI {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        // Paragraph::new(format!("counter: {}", self.counter)).render(area, buf);
        let items = vec![
            Line::from("Item 1"),
            Line::from("Item 2"),
            Line::from("Item 3"),
            Line::from("Item 4"),
            Line::from("Item 5"),
            Line::from("Item 6"),
            Line::from("Item 7"),
            Line::from("Item 8"),
            Line::from("Item 9"),
        ];
        
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state = ScrollbarState::new(items.len()).position(self.vertical_scroll);

        scrollbar.render(area, buf, &mut scrollbar_state);
        Paragraph::new(items).scroll((self.vertical_scroll as u16, 0)).render(area, buf);
    }
}
