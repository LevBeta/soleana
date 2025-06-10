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
        _: &[Pubkey],
        data: &[u8],
    ) -> SoleanaResult<KaminoInstructions> {
        match data[0..8] {
            [0xf2, 0x23, 0xc6, 0x89, 0x52, 0xe1, 0xf2, 0xb6] => Ok(KaminoInstructions::Deposit {
                token_max_a: u64::from_le_bytes(data[8..16].try_into().unwrap()),
                token_max_b: u64::from_le_bytes(data[16..24].try_into().unwrap()),
            }),
            _ => Err(SoleanaError::InvalidInstruction),
        }
    }
}

#[derive(Debug)]
pub enum KaminoInstructions {
    Deposit { token_max_a: u64, token_max_b: u64 },
}

impl ProgramInstructions for KaminoInstructions {}

fn main() {
    let mut parser = TransactionsParser::new();
    parser.register_program::<Kamino>();
    parser.check_programs();
    let x = parser.parse_transaction("014cb7af9d5433b0cb2c863ff3b1a0841a8663140fc662cb74db859a7a219335b9c30437f2dde6f3655c8eafad3428ad28f20123f7fa9af0d8b75980f517a5d2098001000408be1062ccdbdc5e3622f75d3889543d40e69df079ba3d834d4b85be1b16b7cf7f838e6b476c2027750d0a4bb056eb65604ab7390c8d99b78a02ee00664c24868ef88f0011e23a6e1d3f1365746b80800cfd301f0e6b4b7ef9db2db9f3cf3b363b918ce3e5c6b77c49b2a5771ca134fee03bdd791e3d0136e9de22c70b74a4b0d50306466fe5211732ffecadba72c39be7bc8ce5bbc5f7126b2c439b3a400000004f6285b8dcb2f6ab9ff45714692c3ae61dea15d54e7bcf818b1e70e006513d030c8714af393dd4c8e1542a5390c5be91f8b31a628a1034d90fc7bba67afd806822dd40abaef2d90828cc07b4852af22ccad330d6dbb60783c23fbf40553eaeb5981c00c61fb7fdeb13cc69e604d0d64db805902c77b72e5cf27787d1434c42c70304000903a08601000000000004000502c05c1500051700080c12061307090a140102150d030b160e0f1010101118f223c68952e1f2b60039c2000000000064dcb21d00000000028d70b06043526641b3e59321acbd787701c383db4f5e209b598a8614b92725c200060259a6a8080c3b804ff3ca5c0ba8ab2af01c38fa0b9fd6080a5196534bea061fe928ca816f810401070815050425030516");
    println!("{:?}", x);
}
