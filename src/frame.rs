use serde_derive::{Deserialize, Serialize};

use super::Tick;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frame<T: Tick> {
    time: u64,
    tick: T,
}

impl<T: Tick> Frame<T> {
    #[inline(always)]
    pub fn new(
        time: u64,
        tick: T,
    ) -> Frame<T> {
        Frame {
            time,
            tick,
        }
    }

    #[inline(always)]
    pub fn tick(&self) -> &T {
        &self.tick
    }

    #[inline(always)]
    pub fn time(&self) -> u64 {
        self.time
    }

    #[inline(always)]
    pub fn epoch(&self) -> u64 {
        self.tick.epoch(self.time)
    }
}
