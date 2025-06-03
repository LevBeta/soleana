use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use input::InputWidget;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
    DefaultTerminal,
};

mod input;

mod transactions;

mod utils;

use crate::transactions::TransactionWiget;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = SoleanaTui::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct SoleanaTui {
    rpc: String,
    exit: bool,
}

impl SoleanaTui {
    pub fn new() -> Self {
        Self {
            rpc: "https://api.mainnet-beta.solana.com".to_string(),
            exit: false,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::ALT) => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }
}

impl Widget for &SoleanaTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Soleana TUI");

        let instructions = Line::from(vec![" Quit ".into(), "<Alt+Q> ".red().bold().into()]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        block.render(area, buf);

        TransactionWiget.render(area.inner(Margin::new(1, 1)), buf);
    }
}
