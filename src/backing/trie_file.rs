use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::Path;

use radix_trie::{Trie, TrieKey};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct TrieFile<K, V> {
    file: File,
    _phantom: PhantomData<Trie<K, V>>,
}

impl<K, V> TrieFile<K, V>
    where K: Serialize + DeserializeOwned + Clone + TrieKey,
          V: Serialize + DeserializeOwned + Clone + TrieKey
{
    pub fn new<P: AsRef<Path>>(
        path: P,
    ) -> Result<TrieFile<K, V>, io::Error> {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())?;

        Ok(
            TrieFile {
                file,

                _phantom: PhantomData,
            },
        )
    }

    pub fn read(
        &mut self,
    ) -> Option<Trie<K, V>> {
        self.try_read().ok()
    }

    pub fn try_read(
        &mut self,
    ) -> Result<Trie<K, V>, Box<dyn std::error::Error>> {
        self.file
            .seek(
                SeekFrom::Start(0),
            )?;

        let mut buf = Vec::new();

        self.file
            .read_to_end(&mut buf)?;

        Ok(
            bincode::deserialize::<Trie<K, V>>(
                &buf,
            )?
        )
    }

    pub fn write_all(
        &mut self,
        item: &Trie<K, V>,
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
