use std::fmt::Debug;

use super::queueable::Queueable;

pub trait Shuffleable: Debug {}

// Needs to track its own shuffling.
impl Queueable for dyn Shuffleable {}