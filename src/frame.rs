use serde_derive::{Deserialize, Serialize};

use super::Tick;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frame<T: Tick> {
    time: u64,
    tick: Option<T>,
}

impl<T: Tick> Frame<T> {
    pub fn new(
        time: u64,
        tick: Option<T>,
    ) -> Frame<T> {
        Frame {
            time,
            tick,
        }
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn epoch(&self) -> Option<u64> {
        self.tick
            .as_ref()
            .map(|tick|
                     tick.epoch(self.time),
            )
    }
}
