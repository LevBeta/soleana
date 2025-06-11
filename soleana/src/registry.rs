use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

use crate::{
    error::SoleanaResult,
    programs::{Program, ProgramInstructions},
    types::{CompleteAddressLookupTable, Pubkey},
};

pub(crate) type ParserFn = fn(
    program_id: Pubkey,
    ix_accounts: &Vec<u8>,
    data: &[u8],
    accounts: &[Pubkey],
) -> SoleanaResult<Box<dyn ProgramInstructions>>;

pub(crate) struct RegistryInner {
    pub(crate) programs: HashMap<Pubkey, ParserFn>,
    /// We store the luts as a map of account's, trying to keep the same order as the original lut.
    /// Since the transaction returns the index of the account.
    pub(crate) luts: HashMap<Pubkey, Vec<Pubkey>>,
    pub(crate) lut_fetch_fn:
        Option<Box<dyn Fn(&[u8; 32]) -> CompleteAddressLookupTable + Send + Sync>>,
}

static REGISTRY: OnceLock<RwLock<RegistryInner>> = OnceLock::new();

pub(crate) fn registry() -> &'static RwLock<RegistryInner> {
    REGISTRY.get_or_init(|| {
        RwLock::new(RegistryInner {
            programs: HashMap::new(),
            luts: HashMap::new(),
            lut_fetch_fn: None,
        })
    })
}

/// Register's a program to the registry.
pub(crate) fn register_program<P: Program>()
where
    P::Instructions: ProgramInstructions + 'static,
{
    fn wrapper<P: Program>(
        program_id: Pubkey,
        ix_accounts: &Vec<u8>,
        data: &[u8],
        accounts: &[Pubkey],
    ) -> SoleanaResult<Box<dyn ProgramInstructions>>
    where
        P::Instructions: ProgramInstructions + 'static,
    {
        let instruction = P::parse_instruction(program_id, ix_accounts, data, accounts)?;
        Ok(Box::new(instruction))
    }

    let mut registry = registry().write().unwrap();
    registry.programs.insert(P::program_id(), wrapper::<P>);
}

/// Register's a lut to the registry.
pub(crate) fn register_lut(lut: CompleteAddressLookupTable) {
    let mut registry = registry().write().unwrap();
    registry.luts.insert(lut.account_key, lut.accounts);
}
