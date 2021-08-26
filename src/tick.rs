use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Quote {
    pub time: u64,
    pub size: u64,
    pub ask_price: f32,
    pub bid_price: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Trade {
    pub time: u64,
    pub size: u64,
    pub price: f32,
}

pub trait Tick {
    fn time(&self) -> u64;
    fn epoch(&self) -> u64;
}

impl Tick for Quote {
    fn time(&self) -> u64 { self.time }
    fn epoch(&self) -> u64 {
        self.time / 86_400_000_000
    }
}

impl Tick for Trade {
    fn time(&self) -> u64 { self.time }
    fn epoch(&self) -> u64 {
        self.time / 86_400_000_000
    }
}
