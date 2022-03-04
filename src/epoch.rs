use serde::de::DeserializeOwned;
use serde::Serialize;

use super::BackingFile;
use super::frame::Frame;
use super::path_builder::QuotickPathBuilder;
use super::radix_trie::{Trie, TrieCommon};
use super::Tick;

#[derive(Debug)]
pub enum EpochError {
    BackingFileFailure(&'static str),
    IndexFileFailure,
    WriteFailure,
    FrameConflict,
    FrameTooBig,
    FrameEmpty,
}

type FrameIndex<T> = Trie<u64, T>;

pub struct Epoch<T: Tick + Serialize + DeserializeOwned> {
    frame_index_backing: BackingFile<FrameIndex<T>>,

    pub frame_index: FrameIndex<T>,

    epoch: u64,

    tainted: bool,

    path_builder: QuotickPathBuilder,
}

impl<T: Tick + Serialize + DeserializeOwned> Epoch<T> {
    #[inline(always)]
    pub fn new(
        epoch: u64,
        path_builder: QuotickPathBuilder,
    ) -> Result<Epoch<T>, EpochError> {
        let mut frame_index_backing =
            BackingFile::<FrameIndex<T>>::new(
                path_builder.index_backing_file(epoch),
            )
                .or_else(|_|
                    Err(
                        EpochError::BackingFileFailure(
                            "Failed to open frame index backing file.",
                        ),
                    )
                )?;

        let frame_index =
            frame_index_backing.try_read()
                .unwrap_or_else(|_| Trie::new());

        Ok(
            Epoch {
                frame_index_backing,
                frame_index,

                epoch,
                tainted: false,

                path_builder,
            },
        )
    }

    #[inline(always)]
    pub fn frames(&mut self) -> impl Iterator<Item=Frame<T>> + '_ {
        self.frame_index
            .iter()
            .map(|(time, item)|
                     Frame::new(
                         *time,
                         item.clone(),
                     ),
            )
    }

    #[inline(always)]
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    #[inline(always)]
    pub fn insert(
        &mut self,
        frame: &Frame<T>,
        force_overwrite: bool,
    ) -> Result<(), EpochError> {
        let time = frame.time();

        if !force_overwrite && self.frame_index.get(&time).is_some() {
            return Err(EpochError::FrameConflict);
        }

        self.frame_index
            .insert(
                time,
                frame.tick().clone(),
            );

        self.tainted = true;

        Ok(())
    }

    #[inline(always)]
    pub fn persist(&mut self) {
        if !self.tainted {
            return;
        }

        self.frame_index_backing
            .write_all(
                &self.frame_index,
            );

        self.tainted = false;
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for Epoch<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.persist();
    }
}
