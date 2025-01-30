use std::fmt::Debug;

use derive_more::derive::{Display, Error};

pub trait Playable: Debug {}

#[derive(Debug, Display, Error, PartialEq)]
pub enum PlayError {
    FailedLoad
}