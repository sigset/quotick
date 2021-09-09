use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex, PoisonError};

use radix_trie::TrieCommon;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::backing::backing_file::BackingFile;
use super::epoch::Epoch;
use super::frame::Frame;
use super::frameset::FrameSetError;
use super::path_builder::QuotickPathBuilder;
use super::Tick;

#[derive(Debug)]
pub enum EpochBridgeError {
    FrameSet(FrameSetError),
    BackingFileFailure,
    BadFrameEpoch,
    BadFrameTick,
    Inconsistency,
}

impl From<FrameSetError> for EpochBridgeError {
    fn from(err: FrameSetError) -> Self {
        EpochBridgeError::FrameSet(err)
    }
}

pub struct EpochBridge<'a, T: Tick + Serialize + DeserializeOwned> {
    epoch_index_backing: BackingFile<Vec<u64>>,

    pub(crate) epoch_index: Vec<u64>,
    pub(crate) epoch_bridge_iter: Box<dyn Iterator<Item = &'a u64> + 'a>,

    pub(crate) curr_epoch: (u64, Option<Epoch<T>>),

    path_builder: QuotickPathBuilder,
}

impl<'a, T: Tick + Serialize + DeserializeOwned> EpochBridge<'a, T> {
    pub fn new(
        path_builder: QuotickPathBuilder,
    ) -> Result<EpochBridge<'a, T>, EpochBridgeError> {
        let mut epoch_index_backing =
            BackingFile::<Vec<u64>>::new(
                path_builder.epoch_index_backing_file(),
            )
                .map_err(|_| EpochBridgeError::BackingFileFailure)?;

        let mut epoch_index =
            epoch_index_backing.try_read()
                .unwrap_or_else(|_| Vec::new());

        Ok(
            EpochBridge {
                epoch_index_backing,

                epoch_index,
                epoch_bridge_iter: Box::new(
                    epoch_index
                        .iter(),
                ),

                path_builder: path_builder.clone(),

                curr_epoch: (0u64, None),
            },
        )
    }

    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), EpochBridgeError> {
        let frame_epoch =
            frame.epoch()
                .ok_or(EpochBridgeError::BadFrameEpoch)?;

        if self.needs_epoch_update(frame_epoch) {
            self.load_epoch(
                frame_epoch,
            )?;
        }

        let mut curr_epoch =
            &mut self.curr_epoch;

        let ref mut frame_set =
            curr_epoch.1
                .as_mut()
                .ok_or(EpochBridgeError::BadFrameTick)?;

        frame_set.insert(frame);

        Ok(())
    }

    #[inline(always)]
    fn needs_epoch_update(
        &self,
        epoch: u64,
    ) -> bool {
        let curr_epoch = &self.curr_epoch;

        let epoch_mismatch = epoch != curr_epoch.0;
        let need_epoch = curr_epoch.1.is_none();

        epoch_mismatch || need_epoch
    }

    #[inline(always)]
    pub fn load_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<(), EpochBridgeError> {
        self.curr_epoch =
            (
                epoch,
                Some(
                    Epoch::new(
                        epoch,
                        self.path_builder.clone(),
                    )?,
                ),
            );

        self.insert_epoch(
            epoch,
        )?;

        Ok(())
    }

    pub fn insert_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<(), EpochBridgeError> {
        let epoch_index =
            &mut self.epoch_index;

        let bin_search =
            epoch_index.binary_search(&epoch);

        match bin_search {
            Ok(_) => {} // already exists
            Err(pos) => {
                epoch_index
                    .insert(
                        pos,
                        epoch,
                    );
            }
        }

        Ok(())
    }

    pub fn persist(&mut self) -> Result<(), EpochBridgeError> {
        let mut epoch_index = &mut self.epoch_index;
        let mut curr_epoch = &mut self.curr_epoch;

        self.epoch_index_backing
            .write_all(
                &epoch_index,
            );

        if let Some(ref mut epoch) = curr_epoch.1 {
            epoch.persist();
        }

        Ok(())
    }
}

impl<'a, T: Tick + Serialize + DeserializeOwned> Drop for EpochBridge<'a, T> {
    fn drop(&mut self) {
        self.persist();
    }
}
