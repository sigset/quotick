use std::io::SeekFrom;
use std::marker::PhantomData;
use std::path::Path;

use radix_trie::{Trie, TrieCommon};
use serde::Serialize;

use super::config::{build_frame_backing_file_name, build_index_backing_file_name};
use super::frame::Frame;
use super::random_access_file::RandomAccessFile;
use super::Tick;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum FrameSetError {
    BackingFileFailure,
    IndexFileFailure,
    WriteFailure,
    FrameConflict,
    FrameEmpty,
}

pub struct FrameSet<'a, T: Tick> {
    frame_backing_file: RandomAccessFile,
    index_backing_file: RandomAccessFile,
    index: Trie<u64, u64>,
    _phantom: &'a PhantomData<T>,
}

impl<'a, T: Tick> FrameSet<'a, T> {
    pub fn new(
        epoch: u64,
    ) -> Result<FrameSet<'a, T>, FrameSetError> {
        let frame_backing_file =
            RandomAccessFile::new(
                build_frame_backing_file_name(
                    epoch,
                ),
            )
                .or_else(|_| Err(FrameSetError::BackingFileFailure))?;

        let mut index_backing_file =
            RandomAccessFile::new(
                build_frame_backing_file_name(
                    epoch,
                ),
            )
                .or_else(|_| Err(FrameSetError::BackingFileFailure))?;

        let index =
            index_backing_file.read_all::<Trie<u64, u64>>()
                .unwrap_or_else(|_| Trie::new());

        Ok(
            FrameSet {
                frame_backing_file,
                index_backing_file,
                index,
                _phantom: &PhantomData,
            },
        )
    }

    pub(crate) fn frame_backing_file(
        &'a mut self,
    ) -> &'a mut RandomAccessFile {
        &mut self.frame_backing_file
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

        if self.index.get(&time).is_some() {
            return Err(FrameSetError::FrameConflict);
        }

        let offset =
            self.index_backing_file
                .append(&frame)
                .map_err(|_| FrameSetError::WriteFailure)?;

        self.index
            .insert(
                time,
                offset,
            );

        Ok(())
    }

    pub fn iter(&'a mut self) -> FrameSetIter<'a, T> {
        FrameSetIter::new(
            self.index.iter(),
            &mut self.frame_backing_file,
        )
    }
}

impl<'a, T: Tick> Drop for FrameSet<'a, T> {
    fn drop(&mut self) {
        self.index_backing_file
            .write(
                SeekFrom::Start(0),
                &self.index,
            );
    }
}

pub struct FrameSetIter<'a, T> {
    iter: radix_trie::iter::Iter<'a, u64, u64>,
    backing_file: &'a mut RandomAccessFile,
    _phantom: PhantomData<T>,
}

impl<'a, T: Tick> FrameSetIter<'a, T> {
    pub fn new(
        iter: radix_trie::iter::Iter<'a, u64, u64>,
        backing_file: &'a mut RandomAccessFile,
    ) -> FrameSetIter<'a, T> {
        FrameSetIter {
            iter,
            backing_file,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: Tick + Serialize + DeserializeOwned + Default> Iterator for FrameSetIter<'a, T> {
    type Item = Frame<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, offset) =
            match self.iter.next() {
                Some(entry) => entry,
                None => {
                    return None.into();
                }
            };

        let item =
            self.backing_file
                .read::<T>(*offset)
                .ok();

        Some(item.into())
    }
}
