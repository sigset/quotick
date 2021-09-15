use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Quote {
    pub size: u64,
    pub ask_price: f32,
    pub bid_price: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Trade {
    pub size: u64,
    pub price: f32,
}

pub trait Tick: Clone + std::fmt::Debug {
    fn epoch(&self, time: u64) -> u64;
}

impl Tick for Quote {
    fn epoch(&self, time: u64) -> u64 {
        time / 86_400_000_000_000
    }
}

impl Tick for Trade {
    fn epoch(&self, time: u64) -> u64 {
        time / 86_400_000_000_000
    }
}
