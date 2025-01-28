use std::{collections::BTreeSet, sync::Arc};

use crate::queue::{playable::Playable, shuffleable::Shuffleable};

use super::playlist::Playlist;

#[derive(Debug)]
pub struct SortedPlaylist {
    name: String,
    items: BTreeSet<Arc<dyn Playable>> // Sorting passes through the Arc
}

impl SortedPlaylist {
    /// Returns the [`SortedPlaylist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`SortedPlaylist`]'s [`Playable`] items.
    pub fn items(&self) -> &BTreeSet<Arc<dyn Playable>> {
        &self.items
    }
}

impl Playable for SortedPlaylist {} // ? Does this make sense

impl Shuffleable for SortedPlaylist {}

impl Playlist for SortedPlaylist {}