use std::time::Duration;

use bincode;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    if let Ok(mut raf) = quotick::random_access_file::RandomAccessFile::new("./foo") {
        dbg!(raf.append(Pee { a: 1u32, b: 2u64 }));
        dbg!(raf.read::<Pee>(0u64));
    }

    let mut vt = quotick::veb_tree::VEBTree::new(1_000_000).unwrap();

    vt.insert(123);
    vt.insert(657);
    vt.insert(120);
    vt.insert(126);
    vt.insert(106);
    vt.insert(142);
    vt.insert(127);

    dbg!(vt.find_next(124));
}
