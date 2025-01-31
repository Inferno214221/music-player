use std::sync::Arc;

use derive_more::derive::Display;

use super::{executable::{Executable, PlayError}, player::Player, queueable::Queueable};

#[derive(Debug, Display, Clone, Copy)]
pub struct QueuePause;

impl Queueable for QueuePause {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        vec![Arc::new(*self)]
    }
}

impl Executable for QueuePause {
    fn exec(&self, player: &mut Player) -> Result<(), PlayError> {
        player.controller().as_mut().ok_or(PlayError::FailedLoad)?.set_paused(true);
        Ok(())
    }
}

#[derive(Debug, Display, Clone, Copy)]
pub struct QueueStop;

impl Queueable for Arc<QueueStop> {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        vec![self.clone()]
    }
}

impl Executable for QueueStop {
    fn exec(&self, player: &mut Player) -> Result<(), PlayError> {
        player.manager().clear();
        Ok(())
    }
}