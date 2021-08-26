use serde_derive::{Deserialize, Serialize};

use super::Tick;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frame<T: Tick> {
    tick: T,
}

impl<T: Tick> Frame<T> {
    pub fn new(tick: T) -> Frame<T> {
        Frame {
            tick,
        }
    }

    pub fn time(&self) -> u64 {
        self.tick.time()
    }

    pub fn epoch(&self) -> u64 {
        self.tick.epoch()
    }
}
