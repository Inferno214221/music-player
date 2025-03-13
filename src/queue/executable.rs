use std::fmt::{Debug, Display};

use derive_more::derive::{self, Error};

use super::player::Player;

pub trait Executable: Display + Debug + Send + Sync {
    fn exec(&self, player: &mut Player) -> Result<(), PlayError>;
}

#[derive(Debug, derive::Display, Error, PartialEq)]
pub enum PlayError {
    FailedLoad,
}