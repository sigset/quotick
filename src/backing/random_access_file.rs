use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct RandomAccessFile<T: Serialize + DeserializeOwned> {
    file: File,
    _phantom: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> RandomAccessFile<T> {
    pub fn new(
        path: impl AsRef<Path>,
    ) -> Result<RandomAccessFile<T>, io::Error> {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())?;

        Ok(
            RandomAccessFile {
                file,

                _phantom: PhantomData,
            },
        )
    }

    // singular ops

    pub fn read(
        &mut self,
        offset: u64,
        size: u64,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.file
            .seek(
                SeekFrom::Start(
                    offset,
                ),
            )?;

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
            .read(&mut buf)?;

        Ok(
            bincode::deserialize::<T>(
                dbg!(&buf),
            )?
        )
    }

    pub fn write(
        &mut self,
        position: SeekFrom,
        item: &T,
    ) -> Result<(u64, u64), Box<dyn std::error::Error>> {
        let end_pos =
            self.file
                .seek(
                    position,
                )?;

        let buf =
            bincode::serialize(
                item,
            )?;

        self.file
            .write(&buf)?;

        Ok((end_pos, buf.len() as u64))
    }

    pub fn append(
        &mut self,
        item: &T,
    ) -> Result<(u64, u64), Box<dyn std::error::Error>> {
        self.write(
            SeekFrom::End(
                0,
            ),
            item,
        )
    }

    pub fn file_size(
        &mut self,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let metadata = self.file.metadata()?;

        Ok(metadata.len())
    }

    pub fn set_len(
        &mut self,
        new_len: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.file
            .set_len(new_len)?;

        Ok(())
    }

    pub fn truncate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_len(0)
    }
}
