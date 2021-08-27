use std::io::SeekFrom;

use radix_trie::Trie;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::backing::random_access_file::RandomAccessFile;
use super::config::build_epoch_index_backing_file_name;
use super::epoch::Epoch;
use super::frameset::FrameSetError;
use super::Tick;
use super::backing::trie_file::TrieFile;

#[derive(Debug)]
pub enum QuotickError {
    FrameSet(FrameSetError),
    BackingFileFailure,
    Inconsistency,
}

pub type EpochIndex = Trie<u64, u64>;

pub struct Quotick<T: Tick + Serialize + DeserializeOwned> {
    epoch_index_backing: TrieFile<u64, u64>,
    epoch_index: EpochIndex,

    curr_epoch: (u64, Option<Epoch<T>>),
}

impl<T: Tick + Serialize + DeserializeOwned> Quotick<T> {
    pub fn new() -> Result<Quotick<T>, QuotickError> {
        let mut epoch_index_backing =
            TrieFile::<u64, u64>::new(
                build_epoch_index_backing_file_name(),
            )
                .map_err(|_| QuotickError::BackingFileFailure)?;

        let epoch_index =
            epoch_index_backing.try_read()
                .unwrap_or_else(|_| Trie::new());

        Ok(
            Quotick {
                epoch_index_backing,
                epoch_index,

                curr_epoch: (0u64, None),
            },
        )
    }

    pub(crate) fn load_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<(), QuotickError> {
        if self.curr_epoch.0 == epoch {
            return Ok(());
        }

        self.curr_epoch = (epoch, Some(Epoch::new(epoch)?));

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

impl<T: Tick + Serialize + DeserializeOwned> Drop for Quotick<T> {
    fn drop(&mut self) {
        self.persist();
    }
}
