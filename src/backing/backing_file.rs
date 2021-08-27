use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct BackingFile<T> {
    file: File,
    _phantom: PhantomData<T>,
}

impl<T> BackingFile<T>
    where T: Serialize + DeserializeOwned + Clone
{
    pub fn new<P: AsRef<Path>>(
        path: P,
    ) -> Result<BackingFile<T>, io::Error> {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())?;

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
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.file
            .seek(
                SeekFrom::Start(0),
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

    pub fn write_all(
        &mut self,
        item: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.file
            .seek(
                SeekFrom::Start(0),
            )?;

        let buf =
            bincode::serialize(
                item,
            )?;

        self.file
            .write(&buf)?;

        Ok(())
    }
}
