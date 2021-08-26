use std::io::SeekFrom;
use std::marker::PhantomData;
use std::path::Path;

use btree::btree::BTree;
use radix_trie::{Trie, TrieCommon};
use serde::Serialize;

use super::config::{build_frame_backing_file_name, build_index_backing_file_name};
use super::frame::Frame;
use super::random_access_file::RandomAccessFile;
use super::Tick;

#[derive(Debug)]
pub enum FrameSetError {
    BackingFileFailure,
    IndexFileFailure,
    WriteFailure,
    FrameConflict,
}

pub struct FrameSet {
    frame_backing_file: RandomAccessFile,
    index_backing_file: RandomAccessFile,
    index: Trie<u64, u64>,
}

impl FrameSet {
    pub fn new(
        epoch: u64,
    ) -> Result<FrameSet, FrameSetError> {
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
            },
        )
    }

    pub fn insert<T: Tick + Serialize>(
        &mut self,
        frame: Frame<T>,
    ) -> Result<(), FrameSetError> {
        let time = frame.time();

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

    pub fn iter(&self) {}
}

impl Drop for FrameSet {
    fn drop(&mut self) {
        self.index_backing_file
            .write(
                SeekFrom::Start(0),
                &self.index,
            );
    }
}

struct FrameSetIter<'a, T> {
    iter: radix_trie::iter::Iter<'a, u64, u64>,
    _phantom: PhantomData<T>,
}

impl<'a, T: Tick> Iterator for FrameSetIter<'a, T> {
    type Item = Frame<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> Into<FrameSetIter<'a, T>> for FrameSet {
    fn into(self) -> FrameSetIter<'a, T> {
        FrameSetIter {
            iter: self.index.iter(),
            _phantom: PhantomData,
        }
    }
}
