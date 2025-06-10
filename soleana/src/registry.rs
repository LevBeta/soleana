use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

use crate::{
    error::SoleanaResult,
    programs::{Program, ProgramInstructions},
    types::Pubkey,
};

pub(crate) type ParserFn = fn(
    program_id: Pubkey,
    accounts: &[Pubkey],
    data: &[u8],
) -> SoleanaResult<Box<dyn ProgramInstructions>>;

static REGISTRY: OnceLock<RwLock<HashMap<Pubkey, ParserFn>>> = OnceLock::new();

pub(crate) fn registry() -> &'static RwLock<HashMap<Pubkey, ParserFn>> {
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

pub(crate) fn register_program<P: Program>()
where
    P::Instructions: ProgramInstructions + 'static,
{
    fn wrapper<P: Program>(
        program_id: Pubkey,
        accounts: &[Pubkey],
        data: &[u8],
    ) -> SoleanaResult<Box<dyn ProgramInstructions>>
    where
        P::Instructions: ProgramInstructions + 'static,
    {
        let instruction = P::parse_instruction(program_id, accounts, data)?;
        Ok(Box::new(instruction))
    }

    registry()
        .write()
        .unwrap()
        .insert(P::program_id(), wrapper::<P>);
}
