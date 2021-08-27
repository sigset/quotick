use radix_trie::Trie;
use serde_derive::{Deserialize, Serialize};

use quotick::tick::Trade;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Pee {
    a: u32,
    b: u64,
}

fn main() {
//    let mut tick = quotick::Quotick::new(
//        "./raep",
//        "AAPL",
//        quotick::TableType::Quote,
//    ).unwrap();

    let _xy =
        quotick
        ::frameset
        ::FrameSet::<Trade>
        ::new(123)
            .unwrap();

    let mut x: Trie<u64, u64> = Trie::new();

    x.insert(123, 456);
    x.insert(122, 456);
    x.insert(124, 456);

    dbg!(bincode::serialize(&x));
}
