use std::fmt::Debug;

use super::playable::Playable;

pub trait Shuffleable: Playable + Debug {}