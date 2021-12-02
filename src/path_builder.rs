use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct QuotickPathBuilder {
    asset_path: PathBuf,
    base_path: PathBuf,
    epoch_path: PathBuf,
}

impl QuotickPathBuilder {
    #[inline(always)]
    pub fn new(
        asset: &str,
        path: impl AsRef<Path>,
    ) -> QuotickPathBuilder {
        let base_path = path.as_ref().to_path_buf();
        let asset_path = base_path.join(asset);
        let epoch_path = asset_path.join("epoch");

        QuotickPathBuilder {
            base_path: base_path.to_path_buf(),
            asset_path: asset_path.to_path_buf(),
            epoch_path: epoch_path.to_path_buf(),
        }
    }

    #[inline(always)]
    pub fn base_path(&self) -> PathBuf { self.base_path.to_path_buf() }

    #[inline(always)]
    pub fn asset_path(&self) -> PathBuf { self.asset_path.to_path_buf() }

    #[inline(always)]
    pub fn epoch_path(&self) -> PathBuf { self.epoch_path.to_path_buf() }

    #[inline(always)]
    pub fn index_backing_file(
        &self,
        epoch: u64,
    ) -> PathBuf {
        self.epoch_path
            .join(
                &format!(
                    "{}.qtf",
                    epoch,
                ),
            )
            .to_path_buf()
    }

    #[inline(always)]
    pub fn epoch_index_backing_file(
        &self,
    ) -> PathBuf {
        self.asset_path
            .join(
                "epochs.qtf"
            )
            .to_path_buf()
    }
}
