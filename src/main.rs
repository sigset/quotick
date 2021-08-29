use serde_derive::{Deserialize, Serialize};

use quotick::quotick::Quotick;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Trade {
    time: u64,
    size: u32,
    price: u32,
}

impl quotick::tick::Tick for Trade {
    fn time(&self) -> u64 {
        self.time
    }

    fn epoch(&self) -> u64 {
        // one day
        self.time / 86_400_000_000
    }
}

fn main() {
    let trade1 =
        Trade {
            time: 10,
            size: 1,
            price: 2,
        };

    let trade2 =
        Trade {
            time: 11,
            size: 2,
            price: 3,
        };

    let trade3 =
        Trade {
            time: 12,
            size: 3,
            price: 4,
        };

    let quotick =
        Quotick::<Trade>::new(
            "SYMBL",
            "./db",
        );

    if let Ok(mut qt) = quotick {
        qt.insert(&trade1.into());
        qt.insert(&trade2.into());
        qt.insert(&trade3.into());
    }
}
