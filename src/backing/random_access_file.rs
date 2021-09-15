use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;

use miniz_oxide::inflate::TINFLStatus;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub enum RandomAccessFileError {
    External(Box<dyn std::error::Error>),
    IoError(io::Error),
    InflateError(TINFLStatus),
    BadData,
}

pub struct RandomAccessFile<T: Serialize + DeserializeOwned> {
    file: File,
    _phantom: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> RandomAccessFile<T> {
    #[inline(always)]
    pub fn new(
        path: impl AsRef<Path>,
    ) -> Result<RandomAccessFile<T>, RandomAccessFileError> {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())
                .map_err(|err| RandomAccessFileError::IoError(err))?;

        Ok(
            RandomAccessFile {
                file,

                _phantom: PhantomData,
            },
        )
    }

    // singular ops

    #[inline(always)]
    pub fn read(
        &mut self,
        offset: u64,
        size: u64,
    ) -> Result<T, RandomAccessFileError> {
        self.file
            .seek(
                SeekFrom::Start(
                    offset,
                ),
            )
            .map_err(|err| RandomAccessFileError::IoError(err))?;

        let mut buf =
            Vec::with_capacity(
                size as usize,
            );

        buf.resize(
            size as usize,
            0,
        );

        buf.fill(0);

        self.file
            .read(&mut buf)
            .map_err(|err| RandomAccessFileError::IoError(err))?;

        Ok(
            bincode::deserialize::<T>(
                &buf,
            )
                .map_err(|err| RandomAccessFileError::External(err))?
        )
    }

    #[inline(always)]
    pub fn write(
        &mut self,
        position: SeekFrom,
        item: &T,
    ) -> Result<(u64, u64), RandomAccessFileError> {
        let end_pos =
            self.file
                .seek(
                    position,
                )
                .map_err(|err| RandomAccessFileError::IoError(err))?;

        let buf =
            bincode::serialize(
                item,
            )
                .map_err(|err| RandomAccessFileError::External(err))?;

        self.file
            .write(&buf)
            .map_err(|err| RandomAccessFileError::IoError(err))?;

        Ok((end_pos, buf.len() as u64))
    }

    #[inline(always)]
    pub fn append(
        &mut self,
        item: &T,
    ) -> Result<(u64, u64), RandomAccessFileError> {
        self.write(
            SeekFrom::End(
                0,
            ),
            item,
        )
    }

    #[inline(always)]
    pub fn file_size(
        &mut self,
    ) -> Result<u64, RandomAccessFileError> {
        let metadata =
            self.file
                .metadata()
                .map_err(|err| RandomAccessFileError::IoError(err))?;

        Ok(metadata.len())
    }

    #[inline(always)]
    pub fn set_len(
        &mut self,
        new_len: u64,
    ) -> Result<(), RandomAccessFileError> {
        self.file
            .set_len(new_len)
            .map_err(|err| RandomAccessFileError::IoError(err))?;

        Ok(())
    }

    #[inline(always)]
    pub fn truncate(
        &mut self,
    ) -> Result<(), RandomAccessFileError> {
        self.set_len(0)
    }
}
