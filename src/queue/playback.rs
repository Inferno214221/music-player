use std::fmt::Debug;

use awedio::{backends::{CpalBackend, CpalBackendError}, manager::Manager, sounds::wrappers::{Controller, Pausable}, Sound};

pub type OptionalController = Option<Controller<Pausable<Box<dyn Sound>>>>;

pub struct Playback {
    manager: Manager,
    controller: OptionalController,
    _backend: CpalBackend
}

impl Playback {
    pub fn new() -> Result<Playback, CpalBackendError> { // TODO: change error type
        let (manager, _backend) = awedio::start()?;
        Ok(Playback {
            manager,
            controller: None,
            _backend
        })
    }

    pub fn controller(&mut self) -> &mut OptionalController {
        &mut self.controller
    }

    pub fn manager(&mut self) -> &mut Manager {
        &mut self.manager
    }
}

impl Debug for Playback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Playback")
        .field("manager", &self.manager)
        .field("controller", &"&self.controller")
        .field("_backend", &"&self._backend")
        .finish()
    }
}