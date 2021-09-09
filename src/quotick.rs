use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut, Ref};
use std::marker::PhantomData;
use std::path::Path;
use std::rc::Rc;
use std::slice::Iter;
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use radix_trie::TrieCommon;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::Epoch;
use super::epoch_bridge::{EpochBridge, EpochBridgeError};
use super::frame::Frame;
use super::path_builder::QuotickPathBuilder;
use super::Tick;

#[derive(Debug)]
pub enum QuotickError {
    EpochBridge(EpochBridgeError),
    Inconsistency,
}

impl From<EpochBridgeError> for QuotickError {
    fn from(err: EpochBridgeError) -> Self {
        QuotickError::EpochBridge(err)
    }
}

pub fn init_paths(
    path_builder: &QuotickPathBuilder,
) {
    std::fs::create_dir_all(
        path_builder
            .frameset_path(),
    );
}

pub struct Quotick<'a, T: Tick + Serialize + DeserializeOwned> {
    asset: String,

    epoch_bridge: EpochBridge<'a, T>,

    path_builder: QuotickPathBuilder,
}

impl<'a, T: Tick + Serialize + DeserializeOwned> Quotick<'a, T> {
    pub fn new(
        asset: &'a str,
        base_path: impl AsRef<Path>,
    ) -> Result<Quotick<'a, T>, QuotickError> {
        let path_builder =
            QuotickPathBuilder::new(
                &asset,
                base_path,
            );

        init_paths(
            &path_builder,
        );

        let mut epoch_bridge =
            EpochBridge::<T>::new(path_builder.clone())
                .or_else(|err|
                             Err(QuotickError::EpochBridge(err)),
                )?;

        Ok(
            Quotick {
                asset: asset.to_string(),

                epoch_bridge,

                path_builder: path_builder.clone(),
            },
        )
    }

    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), QuotickError> {
        self.epoch_bridge
            .insert(frame)
            .map_err(|err|
                         QuotickError::EpochBridge(err),
            )?;

        Ok(())
    }

    pub fn persist(
        &mut self,
    ) -> Result<(), QuotickError> {
        self.epoch_bridge.persist()?;

        Ok(())
    }

    pub fn index_iter(&'a mut self) -> EpochIndexIter<'a, T> {
        let mut iter =
            EpochIndexIter::<'a, T>::new(
                Box::new(
                    self.epoch_bridge
                        .epoch_index
                        .iter(),
                ),
            );

        iter.set_epoch_bridge(
            &mut self.epoch_bridge,
        );

        iter
    }
}

impl<'a, T: Tick + Serialize + DeserializeOwned> Drop for Quotick<'a, T> {
    fn drop(&mut self) {
        self.persist();
    }
}

pub struct EpochIndexIter<'a, T: Tick + Serialize + DeserializeOwned> {
    epoch_bridge: Option<&'a mut EpochBridge<'a, T>>,
    epoch_index_iter: Box<dyn Iterator<Item=&'a u64> + 'a>,
}

impl<'a, T: Tick + Serialize + DeserializeOwned> EpochIndexIter<'a, T> {
    pub fn new(
        epoch_index_iter: Box<dyn Iterator<Item=&'a u64> + 'a>,
    ) -> Self {
        EpochIndexIter {
            epoch_bridge: None,
            epoch_index_iter,
        }
    }

    pub fn set_epoch_bridge(
        &mut self,
        epoch_bridge: &'a mut EpochBridge<'a, T>,
    ) {
        self.epoch_bridge = Some(epoch_bridge);
    }
}

impl<'a, T: Tick + Serialize + DeserializeOwned> Iterator for EpochIndexIter<'a, T> {
    type Item = Frame<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.epoch_index_iter.next() {
            None => None,
            Some(epoch) => {
                self.epoch_bridge
                    .as_mut()?
                    .load_epoch(*epoch);

                None
            }
        }
    }
}
