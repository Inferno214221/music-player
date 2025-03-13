use std::fmt::Debug;

use super::{queueable::Queueable, shuffled::Shuffled};

pub trait Shuffleable: Queueable + Debug + Sized + Send + Sync {
    fn shuffled(&self) -> Shuffled {
        Shuffled::from(self)
    }
}