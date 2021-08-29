
use std::marker::PhantomData;

use radix_trie::Trie;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::backing::backing_file::BackingFile;
use super::backing::random_access_file::RandomAccessFile;
use super::frame::Frame;
use super::path_builder::QuotickPathBuilder;
use super::Tick;

#[derive(Debug)]
pub enum FrameSetError {
    BackingFileFailure(&'static str),
    IndexFileFailure,
    WriteFailure,
    FrameConflict,
    FrameEmpty,
}

pub type FrameIndex = Trie<u64, u64>;

pub struct FrameSet<T: Tick + Serialize + DeserializeOwned + Default> {
    frame_data_backing: RandomAccessFile<Frame<T>>,
    frame_index_backing: BackingFile<Trie<u64, u64>>,
    frame_index: FrameIndex,
    _phantom: PhantomData<T>,
}

impl<T: Tick + Serialize + DeserializeOwned + Default> FrameSet<T> {
    pub fn new(
        epoch: u64,
        path_builder: &QuotickPathBuilder,
    ) -> Result<FrameSet<T>, FrameSetError> {
        let frame_data_backing =
            RandomAccessFile::<Frame<T>>::new(
                path_builder.frame_backing_file(epoch),
            )
                .or_else(|err| {
                    dbg!(err);

                    Err(
                        FrameSetError::BackingFileFailure(
                            "Failed to open frame backing file.",
                        ),
                    )
                })?;

        let mut frame_index_backing =
            BackingFile::<Trie<u64, u64>>::new(
                path_builder.index_backing_file(epoch),
            )
                .or_else(|_|
                    Err(
                        FrameSetError::BackingFileFailure(
                            "Failed to open frame index backing file.",
                        ),
                    )
                )?;

        let frame_index =
            frame_index_backing.try_read()
                .unwrap_or_else(|_| Trie::new());

        Ok(
            FrameSet {
                frame_data_backing,
                frame_index_backing,
                frame_index,
                _phantom: PhantomData,
            },
        )
    }

    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), FrameSetError> {
        let time =
            frame
                .time()
                .ok_or(
                    FrameSetError::FrameEmpty,
                )?;

        if self.frame_index.get(&time).is_some() {
            return Err(FrameSetError::FrameConflict);
        }

        let offset =
            self.frame_data_backing
                .append(frame)
                .map_err(|_| FrameSetError::WriteFailure)?;

        self.frame_index
            .insert(
                time,
                offset,
            );

        Ok(())
    }

    pub fn persist(&mut self) {
        self.frame_index_backing
            .write_all(
                &self.frame_index,
            );
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for FrameSet<T> {
    fn drop(&mut self) {
        self.persist();
    }
}
