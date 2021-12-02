use std::fs::OpenOptions;
use std::io::Read;

use serde_derive::{Deserialize, Serialize};

use quotick::quotick::Quotick;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Trade {
    size: u32,
    price: u32,
}

impl quotick::tick::Tick for Trade {
    #[inline(always)]
    fn epoch(&self, time: u64) -> u64 {
        // one day
        time / 86_400_000_000_000
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestDataTrade {
    #[serde(rename = "T")]
    T: Option<String>,
    // AAPL
    #[serde(rename = "t")]
    t: i64,
    // 1547787608999125800
    #[serde(rename = "y")]
    y: Option<i64>,
    // 1547787608999125800
    #[serde(rename = "f")]
    f: Option<i64>,
    // 1547787608999125800
    #[serde(rename = "q")]
    q: i64,
    // 23547
    #[serde(rename = "i")]
    i: String,
    // 00MGON
    #[serde(rename = "x")]
    x: i64,
    // 11
    #[serde(rename = "s")]
    s: i64,
    // 100
    #[serde(rename = "c")]
    c: Option<Vec<i64>>,
    #[serde(rename = "p")]
    p: Option<f32>,
    // 223.001
    #[serde(rename = "z")]
    z: i64,              // 1
}

#[inline(always)]
fn run(ticks: &[TestDataTrade]) {
    let mut quotick =
        Quotick::<Trade>::new(
            "SYMBL",
            "./test_data/qt-db",
        )
            .expect("Could not open test database.");

    for tick in ticks {
        if let Some(_) = tick.p {
            //let p = tick.p.unwrap();

            //println!("{} {} {} = ${}", tick.t, tick.s, p, tick.s as f64 * p as f64);
        }

        quotick.insert(
            &quotick::Frame::new(
                tick.t as u64,
                Trade {
                    size: tick.s as u32,
                    price: match tick.p {
                        Some(p) => p as u32,
                        None => { continue; }
                    },
                },
            )
        );
    }

    quotick.persist();

    quotick
        .epochs()
        .for_each(
            |mut epoch| {
                epoch
                    .frames()
                    .for_each(
                        |frame| {
                            frame.time();
                        },
                    );
            }
        );

    dbg!(quotick.oldest_frame());
    dbg!(quotick.newest_frame());
}

fn main() {
    let mut file =
        OpenOptions::new()
            .read(true)
            .write(false)
            .open("./test_data/test_data")
            .expect("Could not open ./test_data/test_data for reading.");

    let mut test_data = Vec::<u8>::new();

    file.read_to_end(&mut test_data);

    let ticks =
        bincode::deserialize::<Vec<TestDataTrade>>(
            &test_data,
        )
            .expect("Could not parse test data.");

    run(&ticks);
}
