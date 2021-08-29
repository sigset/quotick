use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::super::Tick;

pub struct RandomAccessFile<T: Serialize + DeserializeOwned + Default> {
    file: File,
    _phantom: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + Default> RandomAccessFile<T> {
    pub fn new(
        path: impl AsRef<Path>,
    ) -> Result<RandomAccessFile<T>, io::Error> {
        let mut file =
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
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.file
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

        buf.resize(data_size, 0);

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
    ) -> Result<u64, Box<dyn std::error::Error>> {
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

        Ok(end_pos)
    }

    pub fn append(
        &mut self,
        item: &T,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        self.write(
            SeekFrom::End(
                0,
            ),
            item,
        )
    }

    pub fn truncate(
        &self,
    ) {
        self.file.set_len(0);
    }
}
