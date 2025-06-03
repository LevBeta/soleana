use crate::utils::centered_rect;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
};
pub struct InputWidget {
    pub title: String,
    pub input: String,
}

impl Widget for InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(50, 20, area);

        let outer_block = Block::bordered()
            .title(Line::from(self.title))
            .border_set(border::THICK);

        outer_block.render(popup_area, buf);

        let input_area = popup_area.inner(Margin::new(1, 1));

        let input_block = Block::bordered().border_set(border::PLAIN);
        input_block.render(input_area, buf);

        let input_text = Paragraph::new(self.input).alignment(Alignment::Center);
        input_text.render(input_area, buf);
    }
}
