

use radix_trie::{Trie, TrieCommon};
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::backing::random_access_file::RandomAccessFileError;
use super::BackingFile;
use super::frame::Frame;
use super::path_builder::QuotickPathBuilder;
use super::RandomAccessFile;
use super::Tick;

#[derive(Debug)]
pub enum EpochError {
    BackingFileFailure(&'static str),
    RandomAccessFile(RandomAccessFileError),
    IndexFileFailure,
    WriteFailure,
    FrameConflict,
    FrameTooBig,
    FrameEmpty,
}

const EXTENT_SIZE_MASK: &'static u64 = &((u16::MAX as u64) << 48);
const EXTENT_OFFSET_MASK: &'static u64 = &(u64::MAX >> 16);

pub struct FrameExtent {
    pub size: u16,
    // max u16
    pub offset: u64, // max u48
}

impl FrameExtent {
    #[inline(always)]
    pub fn pack(&self) -> u64 {
        // pack u16 size into first 16 bit of u64
        (self.size as u64) << 48
            // pack "u48" offset into last 48 bit of u64
            ^ (self.offset & EXTENT_OFFSET_MASK)
    }

    #[inline(always)]
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

pub struct Epoch<T: Tick + Serialize + DeserializeOwned> {
    frame_data_backing: RandomAccessFile<T>,
    frame_index_backing: BackingFile<FrameIndex>,

    pub frame_index: FrameIndex,

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
        let frame_data_backing =
            RandomAccessFile::<T>::new(
                path_builder.frame_backing_file(epoch),
            )
                .or_else(|_err| {
                    Err(
                        EpochError::BackingFileFailure(
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
                frame_data_backing,
                frame_index_backing,
                frame_index,

                epoch,
                tainted: false,

                path_builder,
            },
        )
    }

    pub fn frame_index_iter(&mut self) -> radix_trie::iter::Iter<u64, u64> {
        self.frame_index
            .iter()
    }

    #[inline(always)]
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    #[inline(always)]
    pub fn frame_backing_file_size(
        &mut self,
    ) -> Result<u64, EpochError> {
        self.frame_data_backing
            .file_size()
            .map_err(|_|
                         EpochError::BackingFileFailure(
                             "Failed to obtain file size of frame backing file.",
                         ),
            )
    }

    #[inline(always)]
    pub fn frame_backing_file_set_len(
        &mut self,
        new_len: u64,
    ) -> Result<(), EpochError> {
        self.frame_data_backing
            .set_len(new_len)
            .map_err(|_|
                         EpochError::BackingFileFailure(
                             "Failed to set file size of frame backing file.",
                         ),
            )
    }

    #[inline(always)]
    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), EpochError> {
        let time = frame.time();

        if self.frame_index.get(&time).is_some() {
            return Err(EpochError::FrameConflict);
        }

        let (offset, size) =
            self.frame_data_backing
                .append(
                    frame.tick()
                        .ok_or(EpochError::FrameEmpty)?
                )
                .map_err(|_| EpochError::WriteFailure)?;

        if size > u16::MAX as u64 {
            let file_size =
                self.frame_backing_file_size()?;

            self.frame_backing_file_set_len(
                file_size - size,
            )?;

            return Err(EpochError::FrameTooBig);
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

        self.tainted = true;

        Ok(())
    }

    #[inline(always)]
    pub fn read_frame(
        &mut self,
        time: u64,
        frame_extent: FrameExtent,
    ) -> Result<Frame<T>, EpochError> {
        let item: T =
            self.frame_data_backing
                .read(
                    frame_extent.offset,
                    frame_extent.size as u64,
                )
                .map_err(|err|
                    EpochError::RandomAccessFile(err)
                )?;

        Ok(
            Frame::new(
                time,
                Some(item),
            ),
        )
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
