use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct RandomAccessFile {
    file: File,
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

        Ok(
            RandomAccessFile {
                file,
            },
        )
    }

    // singular ops

    pub fn read<T: Serialize + DeserializeOwned + Default>(
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

    pub fn write<T: Serialize>(
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

    pub fn append<T: Serialize>(
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

    // file-global ops

    pub fn read_all<T: Serialize + DeserializeOwned>(
        &mut self,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.file
            .seek(
                SeekFrom::Start(
                    0,
                ),
            )?;

        let mut buf = Vec::new();

        self.file
            .read_to_end(&mut buf)?;

        Ok(
            bincode::deserialize::<T>(
                &buf,
            )?
        )
    }

    pub fn truncate(
        &self,
    ) {
        self.file.set_len(0);
    }
}
