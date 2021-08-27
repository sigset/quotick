use std::io::SeekFrom;
use std::marker::PhantomData;

use radix_trie::Trie;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::config::build_frame_backing_file_name;
use super::frame::Frame;
use super::random_access_file::RandomAccessFile;
use super::Tick;

#[derive(Debug)]
pub enum FrameSetError {
    BackingFileFailure(&'static str),
    IndexFileFailure,
    WriteFailure,
    FrameConflict,
    FrameEmpty,
}

pub struct FrameSet<T: Tick + Serialize + DeserializeOwned> {
    frame_data_backing: RandomAccessFile,
    frame_index_backing: RandomAccessFile,
    frame_index: Trie<u64, u64>,
    _phantom: PhantomData<T>,
}

impl<T: Tick + Serialize + DeserializeOwned> FrameSet<T> {
    pub fn new(
        epoch: u64,
    ) -> Result<FrameSet<T>, FrameSetError> {
        let frame_data_backing =
            RandomAccessFile::new(
                build_frame_backing_file_name(
                    epoch,
                ),
            )
                .or_else(|_|
                    Err(
                        FrameSetError::BackingFileFailure(
                            "Failed to open frame backing file.",
                        ),
                    )
                )?;

        let mut frame_index_backing =
            RandomAccessFile::new(
                build_frame_backing_file_name(
                    epoch,
                ),
            )
                .or_else(|_|
                    Err(
                        FrameSetError::BackingFileFailure(
                            "Failed to open frame index backing file.",
                        ),
                    )
                )?;

        let frame_index =
            frame_index_backing.read_all::<Trie<u64, u64>>()
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

    pub fn insert<F: Tick + Serialize>(
        &mut self,
        frame: Frame<F>,
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
            self.frame_index_backing
                .append(&frame)
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
            .write(
                SeekFrom::Start(0),
                &self.frame_index,
            );
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for FrameSet<T> {
    fn drop(&mut self) {
        self.persist();
    }
}
