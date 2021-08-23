use std::error::Error;

use super::config::{build_frame_backing_file_name, build_index_backing_file_name};
use super::random_access_file::RandomAccessFile;
use super::Tick;

pub struct FrameSet {
    frame_backing_file: RandomAccessFile,
    index_backing_file: RandomAccessFile,
}

impl FrameSet {
    pub fn new<T: Tick>(tick: T) -> Result<FrameSet, Box<dyn Error>> {
        let frame_backing_file =
            RandomAccessFile::new(
                build_frame_backing_file_name(
                    &tick,
                ),
            )?;

        let index_backing_file =
            RandomAccessFile::new(
                build_index_backing_file_name(
                    &tick,
                ),
            )?;

        Ok(
            FrameSet {
                frame_backing_file,
                index_backing_file,
            },
        )
    }
}
