use std::fmt::{Debug, Display};
use derive_more::derive::{self, Error};
use super::{queueable::Queueable, player::Player};

pub trait Executable: Queueable + Display + Debug + Send + Sync {
    fn name(&self) -> &str;
    fn exec(&self, player: &mut Player) -> Result<(), PlayError>;
}

#[derive(Debug, derive::Display, Error, PartialEq)]
pub enum PlayError {
    FailedLoad,
    MissingItem
}