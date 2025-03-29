use std::{any::{Any, TypeId}, cmp::Ordering, fmt::Debug, sync::Arc};

use crate::library::{album::Album, artist::Artist, track::Track};

use super::executable::Executable;

pub trait Queueable: Debug + Send + Sync {
    fn executables(&self) -> Vec<Arc<dyn Executable>>;
}

impl PartialEq for dyn Queueable {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for dyn Queueable {}

impl PartialOrd for dyn Queueable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn Queueable {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}