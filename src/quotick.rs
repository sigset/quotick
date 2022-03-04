use std::marker::PhantomData;
use std::path::Path;
use std::slice::Iter;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::Frame;

use super::backing::backing_file::BackingFile;
use super::epoch::Epoch;
use super::epoch::EpochError;
use super::path_builder::QuotickPathBuilder;
use super::Tick;

#[derive(Debug)]
pub enum QuotickError {
    Epoch(EpochError),
    BackingFileFailure,
    BadFrameEpoch,
    BadFrameTick,
    Inconsistency,
}

impl From<EpochError> for QuotickError {
    #[inline(always)]
    fn from(err: EpochError) -> Self {
        QuotickError::Epoch(err)
    }
}

pub fn init_paths(
    path_builder: &QuotickPathBuilder,
) {
    std::fs::create_dir_all(
        path_builder
            .epoch_path(),
    );
}

pub struct Quotick<T: Tick + Serialize + DeserializeOwned> {
    epoch_index_backing: BackingFile<Vec<u64>>,
    pub(crate) epoch_index: Vec<u64>,

    curr_epoch: (u64, Option<Epoch<T>>),

    path_builder: QuotickPathBuilder,

    _phantom: PhantomData<T>,
}

impl<T: Tick + Serialize + DeserializeOwned> Quotick<T> {
    #[inline(always)]
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

        let mut epoch_index_backing =
            BackingFile::<Vec<u64>>::new(
                path_builder.epoch_index_backing_file(),
            )
                .map_err(|_| QuotickError::BackingFileFailure)?;

        let epoch_index =
            epoch_index_backing.try_read()
                .unwrap_or_else(|_| Vec::new());

        Ok(
            Quotick {
                epoch_index_backing,
                epoch_index,

                curr_epoch: (0, None),

                path_builder,

                _phantom: PhantomData,
            },
        )
    }

    #[inline(always)]
    pub fn insert(
        &mut self,
        frame: &Frame<T>,
    ) -> Result<(), QuotickError> {
        let frame_epoch = frame.epoch();

        if self.needs_epoch_update(frame_epoch) {
            self.load_epoch(
                frame_epoch,
            )?;
        }

        let curr_epoch =
            &mut self.curr_epoch;

        let ref mut frame_set =
            curr_epoch.1
                .as_mut()
                .ok_or(QuotickError::BadFrameTick)?;

        frame_set
            .insert(frame)
            .map_err(|err|
                QuotickError::Epoch(err)
            )
    }

    #[inline(always)]
    fn needs_epoch_update(
        &self,
        epoch: u64,
    ) -> bool {
        let curr_epoch = &self.curr_epoch;

        let epoch_mismatch = epoch != curr_epoch.0;
        let need_epoch = curr_epoch.1.is_none();

        epoch_mismatch || need_epoch
    }

    #[inline(always)]
    pub fn load_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<(), QuotickError> {
        self.curr_epoch =
            (
                epoch,
                Some(
                    Epoch::new(
                        epoch,
                        self.path_builder.clone(),
                    )?,
                ),
            );

        self.insert_epoch(
            epoch,
        )
    }

    #[inline(always)]
    pub fn insert_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<(), QuotickError> {
        let epoch_index =
            &mut self.epoch_index;

        let bin_search =
            epoch_index.binary_search(&epoch);

        match bin_search {
            Ok(_) => {} // already exists
            Err(pos) => {
                epoch_index
                    .insert(
                        pos,
                        epoch,
                    );
            }
        }

        Ok(())
    }

    #[inline(always)]
    pub fn persist(&mut self) -> Result<(), QuotickError> {
        let epoch_index = &mut self.epoch_index;
        let curr_epoch = &mut self.curr_epoch;

        self.epoch_index_backing
            .write_all(
                &epoch_index,
            );

        if let Some(ref mut epoch) = curr_epoch.1 {
            epoch.persist();
        }

        Ok(())
    }

    #[inline(always)]
    pub fn oldest_frame(&self) -> Option<Frame<T>> {
        let mut epoch = self.epochs().next()?;

        let res = epoch.frames().next();

        res
    }

    #[inline(always)]
    pub fn newest_frame(&self) -> Option<Frame<T>> {
        let mut epoch =
            Epoch::new(
                self.epoch_index.last().copied()?,
                self.path_builder.clone(),
            ).ok()?;

        epoch.frames().last()
    }

    #[inline(always)]
    pub fn epochs(&self) -> EpochIter<T> {
        EpochIter::<T>::new(
            self.epoch_index.iter(),
            self.path_builder.clone(),
        )
    }
}

impl<T: Tick + Serialize + DeserializeOwned> Drop for Quotick<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.persist();
    }
}

pub struct EpochIter<'a, T: Tick + Serialize + DeserializeOwned> {
    epoch_iter: Iter<'a, u64>,
    curr_epoch: Option<Epoch<T>>,
    path_builder: QuotickPathBuilder,
}

impl<'a, T: Tick + Serialize + DeserializeOwned> EpochIter<'a, T> {
    #[inline(always)]
    pub fn new(
        epoch_iter: Iter<'a, u64>,
        path_builder: QuotickPathBuilder,
    ) -> Self {
        EpochIter {
            epoch_iter,
            curr_epoch: None,
            path_builder,
        }
    }
}

impl<'a, T: 'a + Tick + Serialize + DeserializeOwned> Iterator for EpochIter<'a, T> {
    type Item = Epoch<T>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let epoch = *self.epoch_iter.next()?;

        Epoch::new(
            epoch,
            self.path_builder.clone(),
        )
            .ok()
    }
}
