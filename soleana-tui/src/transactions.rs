use crate::utils::centered_rect;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use solana_message::VersionedMessage;
use soleana::{extended_parse, model::Instruction};

pub struct TransactionWiget {
    tx: String,
}

impl TransactionWiget {
    pub fn new(tx: String) -> Self {
        Self { tx }
    }
}

impl Widget for TransactionWiget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::default()
            .title(Line::from("Transactions"))
            .border_set(border::THICK)
            .borders(Borders::ALL);
        outer_block.clone().render(area, buf);

        let outer_inner = outer_block.inner(area);

        let instructions_block = Block::bordered()
            .title(Line::from("Instructions"))
            .border_set(border::PLAIN)
            .borders(Borders::ALL);

        let instructions_area = centered_rect(80, 80, outer_inner); // or just outer_inner if you want full
        instructions_block.clone().render(instructions_area, buf);

        let transaction = extended_parse(&self.tx).unwrap();
        let instructions: Vec<Line> = transaction
            .instructions()
            .iter()
            .enumerate()
            .flat_map(|(i, instr)| {
                if instr.is_parsed() {
                    let inner = instr.parsed_instruction().unwrap();
                    vec![
                        Line::from(format!("{}. Program: {}", i + 1, inner.program_pubkey())),
                        Line::from(format!("   > Accounts: {:?}", inner.accounts())),
                        match inner.instruction() {
                            Instruction::System(sys_instr) => {
                                Line::from(format!("   > Data: {:?}", sys_instr))
                            }
                            _ => Line::from(""),
                        },
                        Line::from(""), // empty line for spacing
                    ]
                } else {
                    vec![]
                }
            })
            .collect();

        let content_area = instructions_block.clone().inner(instructions_area);
        let paragraph = Paragraph::new(instructions).wrap(Wrap { trim: false });
        paragraph.render(content_area, buf);
    }
}
