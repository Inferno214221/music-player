use std::sync::Weak;

use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::playlist::Playlist;

#[derive(Debug)]
pub struct LinearPlaylist {
    name: String,
    items: Vec<Weak<dyn Playable>>
}

impl LinearPlaylist {
    /// Returns the [`LinearPlaylist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`LinearPlaylist`]'s [`Playable`] items.
    pub fn items(&self) -> &Vec<Weak<dyn Playable>> {
        &self.items
    }
}

impl Playable for LinearPlaylist {}

impl Shuffleable for LinearPlaylist {}

impl Playlist for LinearPlaylist {}