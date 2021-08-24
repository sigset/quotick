use serde_derive::{Deserialize, Serialize};

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

    if let Ok(mut raf) = quotick::random_access_file::RandomAccessFile::new("./foo") {
        dbg!(raf.append(Pee { a: 1u32, b: 2u64 }));
        dbg!(raf.read::<Pee>(0u64));
    }

    quotick::frameset::FrameSet::new(123);
}
