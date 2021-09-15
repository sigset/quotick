use serde_derive::{Deserialize, Serialize};

use super::Tick;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frame<T: Tick> {
    time: u64,
    tick: Option<T>,
}

impl<T: Tick> Frame<T> {
    #[inline(always)]
    pub fn new(
        time: u64,
        tick: Option<T>,
    ) -> Frame<T> {
        Frame {
            time,
            tick,
        }
    }

    #[inline(always)]
    pub fn tick(&self) -> Option<&T> {
        self.tick.as_ref()
    }

    #[inline(always)]
    pub fn time(&self) -> u64 {
        self.time
    }

    #[inline(always)]
    pub fn epoch(&self) -> Option<u64> {
        self.tick
            .as_ref()
            .map(|tick|
                     tick.epoch(self.time),
            )
    }
}
