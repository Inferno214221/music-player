use std::sync::Arc;

use crate::queue::{executable::Executable, shuffleable::Shuffleable, queueable::Queueable};

use super::{playlist::Playlist, playlistable::Playlistable};

#[derive(Debug)]
pub struct LinearPlaylist {
    name: String,
    items: Vec<Arc<dyn Playlistable>>
}

impl LinearPlaylist {
    /// Returns the [`LinearPlaylist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`LinearPlaylist`]'s [`Playable`] items.
    pub fn items(&self) -> &Vec<Arc<dyn Playlistable>> {
        &self.items
    }
}

impl Queueable for LinearPlaylist {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        todo!()
    }
}

impl Shuffleable for LinearPlaylist {}

impl Playlist for LinearPlaylist {}