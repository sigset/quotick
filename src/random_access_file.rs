use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub struct RandomAccessFile {
    pub(crate) file: File,
    pub(crate) reader: BufReader<File>,
    pub(crate) writer: BufWriter<File>,
}

impl RandomAccessFile {
    pub fn new(
        path: impl AsRef<Path>,
    ) -> Result<RandomAccessFile, io::Error> {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())?;

        let reader =
            BufReader::<File>::new(
                file.try_clone()?,
            );

        let writer =
            BufWriter::<File>::new(
                file.try_clone()?,
            );

        Ok(
            RandomAccessFile {
                file,
                reader,
                writer,
            },
        )
    }

    pub fn truncate(
        &self,
    ) {
        self.file.set_len(0);
    }

    pub fn read<T: DeserializeOwned>(
        &mut self,
        offset: u64,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.reader
            .seek(
                SeekFrom::Start(
                    offset,
                ),
            )?;

        let data_size = std::mem::size_of::<T>();

        let mut buf =
            Vec::with_capacity(
                data_size,
            );

        unsafe { buf.set_len(data_size); }

        self.reader
            .read(&mut buf)?;

        Ok(
            bincode::deserialize::<T>(
                dbg!(&buf),
            )?
        )
    }

    pub fn write<T: Serialize>(
        &mut self,
        offset: u64,
        item: T,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let end_pos =
            self.writer
                .seek(
                    SeekFrom::Start(
                        offset,
                    )
                )?;

        let buf =
            bincode::serialize(
                &item,
            )?;

        self.writer
            .write(&buf)?;

        Ok(end_pos)
    }

    pub fn append<T: Serialize>(
        &mut self,
        item: T,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let end_pos =
            self.writer
                .seek(
                    SeekFrom::End(
                        0i64,
                    )
                )?;

        let buf =
            bincode::serialize(
                &item,
            )?;

        self.writer
            .write(&buf)?;

        Ok(end_pos)
    }
}
