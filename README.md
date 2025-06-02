# 🌊 Soleana

**Soleana** is a lightweight, lightning-fast parser for [Solana](https://solana.com) transactions, written in Rust. It efficiently decodes raw transaction data into structured, human-readable formats—perfect for explorers, indexers, or blockchain tools.

---
## Usage/Examples

```rust
use soleana::parse;

fn main() {
  let transaction = parse(
    "01c79cc65469fdfcc8fb10150150e33c73220b976162999d1e38a81176de3aaf90af7f39eacbd261932badd65c3551cdac3f1e60585e2c92e3b52f117bac35750680010002040e7698886e86cd5f4faf3ab562b70f97736ffd2c62eaa7bfe194a2021a82d97cbf971b59108b5b85a04fb093f1e21b4e3fd4c4c8f487dd09b95752769f0dd8c300000000000000000000000000000000000000000000000000000000000000000306466fe5211732ffecadba72c39be7bc8ce5bbc5f7126b2c439b3a400000000124ad783cd3b62be732496acc325d8337e80f1fa06d278a9b534f28fe60a4740203000502e8030000020200010c02000000401f00000000000000"
    );

  // Returns a solana_message::VersionedMessage for normalization.
  // V0(
  //  Message {
  //      header: MessageHeader {
  //          num_required_signatures: 1,
  //          num_readonly_signed_accounts: 0,
  //          num_readonly_unsigned_accounts: 2,
  //      },
  //      account_keys: [
  //          "yTbUyfjwGxP1ovgTkK5UPtNa2KqmYUQWieUcS7iuBbq",
  //          "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
  //          "11111111111111111111111111111111",
  //          "ComputeBudget111111111111111111111111111111",
  //      ],
  //      recent_blockhash: "5Tr3VK6SgLCqW8Ru9HcKeYAK1JXTzZjw6mCn2hSXWiP",
  //      instructions: [
  //          CompiledInstruction {
  //              program_id_index: 3,
  //              accounts: [],
  //              data: [2, 232, 3, 0, 0],
  //          },
  //          CompiledInstruction {
  //              program_id_index: 2,
  //              accounts: [0, 1],
  //              data: [2, 0, 0, 0, 64, 31, 0, 0, 0, 0, 0, 0],
  //          },
  //      ],
  //      address_table_lookups: [],
  //  }
  //)


}
```

