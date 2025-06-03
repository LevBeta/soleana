use crate::utils::centered_rect;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use solana_message::VersionedMessage;
use soleana::parse;

pub struct TransactionWiget;

impl TransactionWiget {
    fn get() -> VersionedMessage {
        let transaction = parse(
            "015617e7ba0fc6e4e86019545775db78e3c7c04e3b2a6d9b0f83ebec58beab9d07151f993ad5ff481013ea899ae1ae8da55b152b231f0c6b4f1d0b1ef566257d07010001036c28e64a8638b935c230556c021ba2e70c3143b2f1cf62450bd70e469c72f101fd743f778d183058b24a31e9caa334657028199b51f870d468f8dca7d10278310000000000000000000000000000000000000000000000000000000000000000be46a36f12a8dcc4f69d31072a4435229b6a2eca74e7b29b002c47cd86d5af6a01020200010c0200000020d7a32300000000"
        ).unwrap();
        transaction
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

        let transaction = TransactionWiget::get();
        let instructions: Vec<Line> = transaction
            .instructions()
            .iter()
            .enumerate()
            .flat_map(|(i, instr)| {
                let address = transaction.static_account_keys()[instr.program_id_index as usize];
                vec![
                    Line::from(format!("{}. Program: {}", i + 1, address)),
                    Line::from(format!("   > Accounts: {:?}", instr.accounts)),
                    Line::from(""), // empty line for spacing
                ]
            })
            .collect();

        let content_area = instructions_block.clone().inner(instructions_area);
        let paragraph = Paragraph::new(instructions).wrap(Wrap { trim: false });
        paragraph.render(content_area, buf);
    }
}
