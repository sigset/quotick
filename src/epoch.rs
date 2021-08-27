use serde::de::DeserializeOwned;
use serde::Serialize;

use super::frameset::FrameSet;
use super::quotick::QuotickError;
use super::Tick;

pub struct Epoch<T: Tick + Serialize + DeserializeOwned> {
    frame_set: FrameSet<T>,
    epoch: u64,
}

impl<'a, T: Tick + Serialize + DeserializeOwned> Epoch<T> {
    pub fn new(
        epoch: u64,
    ) -> Result<Epoch<T>, QuotickError> {
        let frame_set =
            FrameSet::new(epoch)
                .map_err(|err|
                    QuotickError::FrameSet(err)
                )?;

        Ok(
            Epoch {
                frame_set,
                epoch,
            },
        )
    }

    pub fn persist(&mut self) {
        self.frame_set
            .persist();
    }
}
