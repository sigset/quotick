use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::{decompress_to_vec, TINFLStatus};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub enum BackingFileError {
    External(Box<dyn std::error::Error>),
    IoError(io::Error),
    InflateError(TINFLStatus),
    BadData,
}

pub struct BackingFile<T> {
    file: File,
    _phantom: PhantomData<T>,
}

impl<T> BackingFile<T>
    where T: Serialize + DeserializeOwned + Clone
{
    pub fn new<P: AsRef<Path>>(
        path: P,
    ) -> Result<BackingFile<T>, BackingFileError> {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())
                    .map_err(|err| BackingFileError::IoError(err))?;

        Ok(
            BackingFile {
                file,
                _phantom: PhantomData,
            },
        )
    }

    pub fn read(
        &mut self,
    ) -> Option<T> {
        self.try_read().ok()
    }

    pub fn try_read(
        &mut self,
    ) -> Result<T, BackingFileError> {
        self.file
            .seek(
                SeekFrom::Start(0),
            )
            .map_err(|err| BackingFileError::IoError(err))?;

        let mut buf = Vec::new();

        self.file
            .read_to_end(
                &mut buf,
            )
            .map_err(|err| BackingFileError::IoError(err))?;

        let decompressed_buf =
            decompress_to_vec(
                &buf,
            )
            .map_err(|err| BackingFileError::InflateError(err))?;

        Ok(
            bincode::deserialize::<T>(
                &decompressed_buf,
            )
            .map_err(|err| BackingFileError::External(err))?
        )
    }

    pub fn write_all(
        &mut self,
        item: &T,
    ) -> Result<(), BackingFileError> {
        self.file
            .seek(
                SeekFrom::Start(0),
            )
            .map_err(|err| BackingFileError::IoError(err))?;

        let buf =
            bincode::serialize(
                item,
            )
            .map_err(|err| BackingFileError::External(err))?;

        let compressed_buf =
            compress_to_vec(
                &buf,
                3,
            );

        self.file
            .write(
                &compressed_buf,
            )
            .map_err(|err| BackingFileError::IoError(err))?;

        Ok(())
    }
}
