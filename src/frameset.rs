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
    FrameTooBig,
    FrameEmpty,
}

const EXTENT_SIZE_MASK: &'static u64 = &((u16::MAX as u64) << 48);
const EXTENT_OFFSET_MASK: &'static u64 = &(u64::MAX >> 16);

struct FrameExtent {
    size: u16,
    // max u16
    offset: u64, // max u48
}

impl FrameExtent {
    pub fn pack(&self) -> u64 {
        // pack u16 size into first 16 bit of u64
        (self.size as u64) << 48
            // pack "u48" offset into last 48 bit of u64
            ^ (self.offset & EXTENT_OFFSET_MASK)
    }

    pub fn unpack(
        value: u64,
    ) -> Self {
        Self {
            size: ((value & EXTENT_SIZE_MASK) >> 48) as u16,
            offset: value & EXTENT_OFFSET_MASK,
        }
    }
}

pub type FrameIndex = Trie<u64, u64>;

pub struct FrameSet<T: Tick + Serialize + DeserializeOwned> {
    frame_data_backing: RandomAccessFile<Frame<T>>,
    frame_index_backing: BackingFile<FrameIndex>,
    frame_index: FrameIndex,
    _phantom: PhantomData<T>,
}

impl<T: Tick + Serialize + DeserializeOwned> FrameSet<T> {
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
            BackingFile::<FrameIndex>::new(
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

    pub fn frame_backing_file_size(
        &mut self,
    ) -> Result<u64, FrameSetError> {
        self.frame_data_backing
            .file_size()
            .map_err(|_|
                         FrameSetError::BackingFileFailure(
                             "Failed to obtain file size of frame backing file.",
                         ),
            )
    }

    pub fn frame_backing_file_set_len(
        &mut self,
        new_len: u64,
    ) -> Result<(), FrameSetError> {
        self.frame_data_backing
            .set_len(new_len)
            .map_err(|_|
                         FrameSetError::BackingFileFailure(
                             "Failed to set file size of frame backing file.",
                         ),
            )
    }

    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), FrameSetError> {
        let time = frame.time();

        if self.frame_index.get(&time).is_some() {
            return Err(FrameSetError::FrameConflict);
        }

        let (offset, size) =
            self.frame_data_backing
                .append(frame)
                .map_err(|_| FrameSetError::WriteFailure)?;

        if size > u16::MAX as u64 {
            let file_size =
                self.frame_backing_file_size()?;

            self.frame_backing_file_set_len(
                file_size - size,
            )?;

            return Err(FrameSetError::FrameTooBig);
        }

        let frame_value =
            FrameExtent {
                offset,
                size: (size & u16::MAX as u64) as u16,
            };

        self.frame_index
            .insert(
                time,
                frame_value.pack(),
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
