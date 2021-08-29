use std::path::{Path};

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::epoch_bridge::{EpochBridge, EpochBridgeError};
use super::path_builder::QuotickPathBuilder;
use super::Tick;
use crate::frame::Frame;

#[derive(Debug)]
pub enum QuotickError {
    EpochBridge(EpochBridgeError),
    Inconsistency,
}

pub fn init_paths(
    path_builder: &QuotickPathBuilder,
) {
    std::fs::create_dir_all(
            path_builder
            .frameset_path(),
    );
}

pub struct Quotick<T: Tick + Serialize + DeserializeOwned> {
    asset: String,
    epoch_bridge: EpochBridge<T>,

    path_builder: QuotickPathBuilder,
}

impl<T: Tick + Serialize + DeserializeOwned> Quotick<T> {
    pub fn new(
        asset: &str,
        base_path: impl AsRef<Path>,
    ) -> Result<Quotick<T>, QuotickError> {
        let path_builder =
            QuotickPathBuilder::new(
                &asset,
                base_path,
            );

        init_paths(
            &path_builder,
        );

        let epoch_bridge =
            EpochBridge::<T>::new(&path_builder)
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
    ) -> Result<(), EpochBridgeError> {
        self.epoch_bridge
            .insert(frame)
    }

    pub fn persist(
        &mut self,
    ) {
        self.epoch_bridge
            .persist();
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for Quotick<T> {
    fn drop(&mut self) {
        self.persist();
    }
}
