use std::io::SeekFrom;

use radix_trie::Trie;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::backing::backing_file::BackingFile;
use super::backing::random_access_file::RandomAccessFile;
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

pub type EpochIndex = Trie<u64, u64>;

pub struct EpochBridge<T: Tick + Serialize + DeserializeOwned> {
    epoch_index_backing: BackingFile<Trie<u64, u64>>,
    epoch_index: EpochIndex,

    path_builder: QuotickPathBuilder,

    curr_epoch: (u64, Option<Epoch<T>>),
}

impl<T: Tick + Serialize + DeserializeOwned> EpochBridge<T> {
    pub fn new(
        path_builder: &QuotickPathBuilder,
    ) -> Result<EpochBridge<T>, EpochBridgeError> {
        let mut epoch_index_backing =
            BackingFile::<Trie<u64, u64>>::new(
                path_builder.epoch_index_backing_file(),
            )
                .map_err(|_| EpochBridgeError::BackingFileFailure)?;

        let epoch_index =
            epoch_index_backing.try_read()
                .unwrap_or_else(|_| Trie::new());

        Ok(
            EpochBridge {
                epoch_index_backing,
                epoch_index,

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

        if self.curr_epoch.1.is_none() || frame_epoch != self.curr_epoch.0 {
            self.load_epoch(
                frame_epoch,
            )?;
        }

        let ref mut frame_set =
            self.curr_epoch.1.as_mut()
                .ok_or(EpochBridgeError::BadFrameTick)?;

        frame_set.insert(frame);

        Ok(())
    }

    pub fn load_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<(), EpochBridgeError> {
        if dbg!(self.curr_epoch.0) == dbg!(epoch) {
            return Ok(());
        }

        self.curr_epoch = (epoch, Some(Epoch::new(epoch, &self.path_builder)?));

        Ok(())
    }

    pub fn persist(&mut self) {
        self.epoch_index_backing
            .write_all(
                &self.epoch_index,
            );

        if let Some(ref mut epoch) = self.curr_epoch.1 {
            epoch.persist();
        }
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for EpochBridge<T> {
    fn drop(&mut self) {
        self.persist();
    }
}
