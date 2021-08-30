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

pub struct EpochBridge<T: Tick + Serialize + DeserializeOwned> {
    epoch_index_backing: BackingFile<Vec<u64>>,
    epoch_index: Vec<u64>,

    path_builder: QuotickPathBuilder,

    curr_epoch: (u64, Option<Epoch<T>>),
}

impl<T: Tick + Serialize + DeserializeOwned> EpochBridge<T> {
    pub fn new(
        path_builder: &QuotickPathBuilder,
    ) -> Result<EpochBridge<T>, EpochBridgeError> {
        let mut epoch_index_backing =
            BackingFile::<Vec<u64>>::new(
                path_builder.epoch_index_backing_file(),
            )
                .map_err(|_| EpochBridgeError::BackingFileFailure)?;

        let epoch_index =
            epoch_index_backing.try_read()
                .unwrap_or_else(|_| Vec::new());

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

        if self.needs_epoch_update(frame_epoch) {
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

    #[inline(always)]
    fn needs_epoch_update(
        &self,
        epoch: u64,
    ) -> bool {
        let epoch_mismatch = epoch != self.curr_epoch.0;
        let need_epoch = self.curr_epoch.1.is_none();

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
                        &self.path_builder,
                    )?
                ),
            );

        self.insert_epoch(
            epoch,
        );

        Ok(())
    }

    pub fn insert_epoch(
        &mut self,
        epoch: u64,
    ) {
        match self.epoch_index.binary_search(&epoch) {
            Ok(_) => {} // already exists
            Err(pos) => {
                self.epoch_index
                    .insert(
                        pos,
                        epoch,
                    );
            }
        }
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
