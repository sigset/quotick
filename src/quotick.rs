use std::io::SeekFrom;

use radix_trie::Trie;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::config::build_epoch_index_backing_file_name;
use super::frameset::FrameSet;
use super::frameset::FrameSetError;
use super::random_access_file::RandomAccessFile;
use super::Tick;

#[derive(Debug)]
pub enum QuotickError {
    FrameSet(FrameSetError),
    BackingFileFailure,
    Inconsistency,
}

pub struct Quotick<T: Tick + Serialize + DeserializeOwned> {
    epoch_index_backing: RandomAccessFile,
    epoch_index: Trie<u64, u64>,

    curr_epoch_frame_set: Option<FrameSet<T>>,
    curr_epoch: u64,
}

impl<T: Tick + Serialize + DeserializeOwned> Quotick<T> {
    pub fn new() -> Result<Quotick<T>, QuotickError> {
        let mut epoch_index_backing =
            RandomAccessFile::new(
                build_epoch_index_backing_file_name(),
            )
                .map_err(|_| QuotickError::BackingFileFailure)?;

        let epoch_index =
            epoch_index_backing.read_all::<Trie<u64, u64>>()
                .unwrap_or_else(|_| Trie::new());

        Ok(
            Quotick {
                epoch_index_backing,
                epoch_index,

                curr_epoch_frame_set: None,
                curr_epoch: 0,
            },
        )
    }

    pub fn persist(&mut self) {
        self.epoch_index_backing
            .write(
                SeekFrom::Start(0),
                &self.epoch_index,
            );

        if let Some(ref mut frame_set) = self.curr_epoch_frame_set {
            frame_set.persist();
        }
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for Quotick<T> {
    fn drop(&mut self) {
        self.persist();
    }
}
