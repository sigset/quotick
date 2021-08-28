use radix_trie::Trie;
use serde_derive::{Deserialize, Serialize};

use quotick::quotick::Quotick;
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

    let trade1 =
        Trade {
            time: 0,
            size: 1,
            price: 2.0,
        };

    let trade2 =
        Trade {
            time: 1,
            size: 2,
            price: 3.0,
        };

    let trade3 =
        Trade {
            time: 2,
            size: 3,
            price: 4.0,
        };

    if let Ok(mut qt) = Quotick::<Trade>::new(
        "AAPL",
        "./db",
    ) {
        qt.insert(&trade1.into());
        qt.insert(&trade2.into());
        qt.insert(&trade3.into());
    }
}
