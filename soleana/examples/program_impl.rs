use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey as SolanaPubkey;
use soleana::{prelude::program_impl::*, TransactionsParser};

struct Kamino;

impl Program for Kamino {
    fn program_id() -> Pubkey {
        [
            79, 98, 133, 184, 220, 178, 246, 171, 159, 244, 87, 20, 105, 44, 58, 230, 29, 234, 21,
            213, 78, 123, 207, 129, 139, 30, 112, 224, 6, 81, 61, 3,
        ]
    }

    type Instructions = KaminoInstructions;

    fn parse_instruction(
        _: Pubkey,
        ix_accounts_indexes: &Vec<u8>,
        data: &[u8],
        accounts: &[Pubkey],
    ) -> SoleanaResult<KaminoInstructions> {
        let accs = Kamino::match_accounts(ix_accounts_indexes, accounts);
        match data[0..8] {
            [0xf2, 0x23, 0xc6, 0x89, 0x52, 0xe1, 0xf2, 0xb6] => Ok(KaminoInstructions::Deposit {
                token_max_a: u64::from_le_bytes(data[8..16].try_into().unwrap()),
                token_max_b: u64::from_le_bytes(data[16..24].try_into().unwrap()),
                accounts: KaminoDepositAccounts {
                    user: accs[0],
                    strategy: accs[1],
                    global_config: accs[2],
                    pool: accs[3],
                    position: accs[4],
                    tick_array_lower: accs[5],
                    tick_array_upper: accs[6],
                    token_a_vault: accs[7],
                    token_b_vault: accs[8],
                    base_vault_authority: accs[9],
                    token_a_ata: accs[10],
                    token_b_ata: accs[11],
                    token_a_mint: accs[12],
                    token_b_mint: accs[13],
                    user_shares_ata: accs[14],
                    shares_mint: accs[15],
                    shares_mint_authority: accs[16],
                    scope_prices: accs[17],
                    token_infos: accs[18],
                    token_program: accs[19],
                    token_a_token_program: accs[20],
                    token_b_token_program: accs[21],
                    instruction_sysvar_account: accs[22],
                },
            }),
            _ => Err(SoleanaError::InvalidInstruction),
        }
    }
}

#[derive(Debug)]
pub enum KaminoInstructions {
    Deposit {
        token_max_a: u64,
        token_max_b: u64,
        accounts: KaminoDepositAccounts,
    },
}

impl ProgramInstructions for KaminoInstructions {}

#[derive(Debug)]
pub struct KaminoDepositAccounts {
    pub user: Pubkey,
    pub strategy: Pubkey,
    pub global_config: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub base_vault_authority: Pubkey,
    pub token_a_ata: Pubkey,
    pub token_b_ata: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub user_shares_ata: Pubkey,
    pub shares_mint: Pubkey,
    pub shares_mint_authority: Pubkey,
    pub scope_prices: Pubkey,
    pub token_infos: Pubkey,
    pub token_program: Pubkey,
    pub token_a_token_program: Pubkey,
    pub token_b_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}

fn main() {
    let mut parser = TransactionsParser::new();
    parser.register_program::<Kamino>();

    let lut = fetch_lut(&[
        141, 112, 176, 96, 67, 82, 102, 65, 179, 229, 147, 33, 172, 189, 120, 119, 1, 195, 131,
        219, 79, 94, 32, 155, 89, 138, 134, 20, 185, 39, 37, 194,
    ]);
    let lut2 = fetch_lut(&[
        59, 128, 79, 243, 202, 92, 11, 168, 171, 42, 240, 28, 56, 250, 11, 159, 214, 8, 10, 81,
        150, 83, 75, 234, 6, 31, 233, 40, 202, 129, 111, 129,
    ]);
    // Register a lut to the parser.
    parser.register_lut(lut);
    parser.register_lut(lut2);

    let x = parser.parse_transaction("014cb7af9d5433b0cb2c863ff3b1a0841a8663140fc662cb74db859a7a219335b9c30437f2dde6f3655c8eafad3428ad28f20123f7fa9af0d8b75980f517a5d2098001000408be1062ccdbdc5e3622f75d3889543d40e69df079ba3d834d4b85be1b16b7cf7f838e6b476c2027750d0a4bb056eb65604ab7390c8d99b78a02ee00664c24868ef88f0011e23a6e1d3f1365746b80800cfd301f0e6b4b7ef9db2db9f3cf3b363b918ce3e5c6b77c49b2a5771ca134fee03bdd791e3d0136e9de22c70b74a4b0d50306466fe5211732ffecadba72c39be7bc8ce5bbc5f7126b2c439b3a400000004f6285b8dcb2f6ab9ff45714692c3ae61dea15d54e7bcf818b1e70e006513d030c8714af393dd4c8e1542a5390c5be91f8b31a628a1034d90fc7bba67afd806822dd40abaef2d90828cc07b4852af22ccad330d6dbb60783c23fbf40553eaeb5981c00c61fb7fdeb13cc69e604d0d64db805902c77b72e5cf27787d1434c42c70304000903a08601000000000004000502c05c1500051700080c12061307090a140102150d030b160e0f1010101118f223c68952e1f2b60039c2000000000064dcb21d00000000028d70b06043526641b3e59321acbd787701c383db4f5e209b598a8614b92725c200060259a6a8080c3b804ff3ca5c0ba8ab2af01c38fa0b9fd6080a5196534bea061fe928ca816f810401070815050425030516");
    println!("{:?}", x);
}

const LOOKUP_TABLE_META_SIZE: usize = 56;
const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

fn fetch_lut(lut_pk: &[u8; 32]) -> ([u8; 32], Vec<[u8; 32]>) {
    let rpc_client = RpcClient::new(RPC_URL.to_string());
    let lut = rpc_client
        .get_account(&SolanaPubkey::new_from_array(*lut_pk))
        .unwrap();

    let addresses = lut.data[LOOKUP_TABLE_META_SIZE..]
        .chunks(32)
        .map(|chunk| chunk.try_into().unwrap())
        .collect();

    (*lut_pk, addresses)
}
