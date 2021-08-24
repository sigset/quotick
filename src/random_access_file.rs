use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

use serde::Serialize;
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

    pub fn read<T: Serialize + DeserializeOwned + Default>(
        &mut self,
        offset: u64,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.reader
            .seek(
                SeekFrom::Start(
                    offset,
                ),
            )?;

        let data_size =
            bincode::serialized_size(
                &T::default(),
            )? as usize;

        let mut buf =
            Vec::with_capacity(
                data_size,
            );

        unsafe { buf.set_len(data_size); }

        buf.fill(0);

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
        position: SeekFrom,
        item: T,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let end_pos =
            self.writer
                .seek(
                    position,
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
        self.write(
            SeekFrom::End(
                0,
            ),
            item,
        )
    }
}
