use std::path::Path;

use btree::btree::BTree;

use super::config::{build_frame_backing_file_name, build_index_backing_file_name};
use super::random_access_file::RandomAccessFile;

pub enum FrameSetError {
    BackingFileFailure,
    IndexFileFailure,
}

pub struct FrameSet {
    frame_backing_file: RandomAccessFile,
    index_backing_file: BTree,
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

        let index_backing_file =
            btree::btree::BTreeBuilder::new()
                .path(
                    Path::new(
                        &build_index_backing_file_name(
                            epoch,
                        ),
                    ),
                )
                .b_parameter(2)
                .build()
                .or_else(|_| Err(FrameSetError::IndexFileFailure))?;

        Ok(
            FrameSet {
                frame_backing_file,
                index_backing_file,
            },
        )
    }
}
