use std::{any::Any, sync::Arc};

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
        if let Some(controller) = player.controller().as_mut() {
            controller.set_paused(true);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "QueuePause"
    }
}

#[derive(Debug, Display, Clone, Copy)]
pub struct QueueStop;

impl Queueable for QueueStop {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        vec![Arc::new(*self)]
    }
}

impl Executable for QueueStop {
    fn exec(&self, player: &mut Player) -> Result<(), PlayError> {
        player.manager().clear();
        Ok(())
    }

    fn name(&self) -> &str {
        "QueueStop"
    }
}