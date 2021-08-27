use serde_derive::{Deserialize, Serialize};

use super::Tick;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frame<T: Tick> {
    tick: Option<T>,
}

impl<T: Tick> Frame<T> {
    pub fn new(tick: Option<T>) -> Frame<T> {
        Frame {
            tick,
        }
    }

    pub fn time(&self) -> Option<u64> {
        self.tick
            .as_ref()
            .map(|tick| tick.time())
    }

    pub fn epoch(&self) -> Option<u64> {
        self.tick
            .as_ref()
            .map(|tick| tick.epoch())
    }
}

impl<T: Tick> From<T> for Frame<T> {
    fn from(item: T) -> Frame<T> {
        Frame::new(Some(item))
    }
}

impl<T: Tick> From<Option<T>> for Frame<T> {
    fn from(item: Option<T>) -> Self {
        Self::new(item)
    }
}

impl<T: Tick> Default for Frame<T> {
    fn default() -> Self {
        Frame::new(
            Some(
                T::default(),
            ),
        )
    }
}
