use std::cell::RefCell;
use std::rc::Rc;

use radix_trie::iter::Iter;
use radix_trie::TrieCommon;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::epoch_bridge::EpochBridgeError;
use super::frame::Frame;
use super::frameset::FrameSet;
use super::path_builder::QuotickPathBuilder;
use super::Tick;

pub struct Epoch<T: Tick + Serialize + DeserializeOwned> {
    frame_set: FrameSet<T>,

    epoch: u64,

    path_builder: QuotickPathBuilder,
}

impl<'a, T: Tick + Serialize + DeserializeOwned> Epoch<T> {
    pub fn new(
        epoch: u64,
        path_builder: QuotickPathBuilder,
    ) -> Result<Epoch<T>, EpochBridgeError> {
        let frame_set =
            FrameSet::new(epoch, path_builder.clone())
                .map_err(|err|
                    EpochBridgeError::FrameSet(err)
                )?;

        Ok(
            Epoch {
                frame_set,

                epoch,

                path_builder,
            },
        )
    }

    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), EpochBridgeError> {
        self.frame_set
            .insert(frame)
            .or_else(|err| Err(EpochBridgeError::FrameSet(err)))?;

        Ok(())
    }

    pub fn persist(&mut self) {
        self.frame_set
            .persist();
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for Epoch<T> {
    fn drop(&mut self) {
        self.persist();
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Iterator for Epoch<T> {
    type Item = Frame<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut frame_set = &mut self.frame_set;

//        match self.frame_set_index_iter.as_ref()?.next() {
//            None => { return None; }
//            Some(val) => {
        None
//            }
//        }
    }
}
