use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use soleana::TransactionsParser;
fn main() {
    let mut parser = TransactionsParser::new();

    // Register a lut fetch function to the parser.
    // This function will only be used when a lut is not found in the parser's registry.
    parser.register_lut_fetch_fn(fetch_lut);

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
}

const LOOKUP_TABLE_META_SIZE: usize = 56;
const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

fn fetch_lut(lut_pk: &[u8; 32]) -> ([u8; 32], Vec<[u8; 32]>) {
    let rpc_client = RpcClient::new(RPC_URL.to_string());
    let lut = rpc_client
        .get_account(&Pubkey::new_from_array(*lut_pk))
        .unwrap();

    let addresses = lut.data[LOOKUP_TABLE_META_SIZE..]
        .chunks(32)
        .map(|chunk| chunk.try_into().unwrap())
        .collect();

    (*lut_pk, addresses)
}
