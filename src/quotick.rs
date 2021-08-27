use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::Tick;
use crate::epoch_bridge::{EpochBridge, EpochBridgeError};
use crate::path_builder::QuotickPathBuilder;

#[derive(Debug)]
pub enum QuotickError {
    EpochBridge(EpochBridgeError),
    Inconsistency,
}

pub struct Quotick<T: Tick + Serialize + DeserializeOwned> {
    asset: String,
    path_builder: QuotickPathBuilder,
    epoch_bridge: EpochBridge<T>,
}

impl<T: Tick + Serialize + DeserializeOwned> Quotick<T> {
    pub fn new(
        asset: String,
        base_path: impl AsRef<Path>,
    ) -> Result<Quotick<T>, QuotickError> {
        let path_builder =
            QuotickPathBuilder::new(
                &asset,
                base_path,
            );

        let epoch_bridge =
            EpochBridge::<T>::new()
                .or_else(|err|
                    Err(QuotickError::EpochBridge(err)),
                )?;

        let quotick =
            Quotick {
                asset,
                path_builder,
                epoch_bridge,
            };

        quotick.init_paths();

        Ok(quotick)
    }

    pub fn init_paths(&self) {
        std::fs::create_dir_all(
            self
                .path_builder
                .frameset_path(),
        );
    }
}
