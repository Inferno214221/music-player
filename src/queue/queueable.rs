use std::{fmt::Debug, sync::Arc};
use super::executable::Executable;

pub trait Queueable: Debug + Send + Sync {
    fn executables(&self) -> Vec<Arc<dyn Executable>>;
}