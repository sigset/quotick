use std::cmp::{max_by, min_by, Ordering};
use std::error::Error;

pub use tick::Tick;

pub mod config;
pub mod frame;
pub mod frameset;
pub mod random_access_file;
pub mod tick;
pub mod quotick;
pub mod veb_tree;
