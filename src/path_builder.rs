use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct QuotickPathBuilder {
    asset_path: PathBuf,
    base_path: PathBuf,
    frameset_path: PathBuf,
}

impl QuotickPathBuilder {
    pub fn new(
        asset: &str,
        path: impl AsRef<Path>,
    ) -> QuotickPathBuilder {
        let base_path = path.as_ref().to_path_buf();
        let asset_path = base_path.join(asset);
        let frameset_path = asset_path.join("frameset");

        QuotickPathBuilder {
            base_path: base_path.to_path_buf(),
            asset_path: frameset_path.to_path_buf(),
            frameset_path: frameset_path.to_path_buf(),
        }
    }

    pub fn base_path(&self) -> PathBuf { self.base_path.to_path_buf() }
    pub fn asset_path(&self) -> PathBuf { self.asset_path.to_path_buf() }
    pub fn frameset_path(&self) -> PathBuf { self.frameset_path.to_path_buf() }

    pub fn frame_backing_file(
        &self,
        epoch: u64,
    ) -> PathBuf {
        self.frameset_path
            .join(
                &format!(
                    "{}.qtf",
                    epoch,
                ),
            )
            .to_path_buf()
    }

    pub fn index_backing_file(
        &self,
        epoch: u64,
    ) -> PathBuf {
        self.frameset_path
            .join(
                &format!(
                    "{}.qti",
                    epoch,
                ),
            )
            .to_path_buf()
    }

    pub fn epoch_index_backing_file(
        &self,
    ) -> PathBuf {
        self.asset_path
            .join(
                "epochs.qti"
            )
            .to_path_buf()
    }
}
