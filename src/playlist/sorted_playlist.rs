use std::{collections::BTreeSet, sync::Weak};

use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::playlist::Playlist;

#[derive(Debug)]
pub struct SortedPlaylist {
    name: String,
    items: BTreeSet<Weak<dyn Playable>> // Sorting passes through the Arc
}

impl SortedPlaylist {
    /// Returns the [`SortedPlaylist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`SortedPlaylist`]'s [`Playable`] items.
    pub fn items(&self) -> &BTreeSet<Weak<dyn Playable>> {
        &self.items
    }
}

impl Playable for SortedPlaylist {} // ? Does this make sense

impl Shuffleable for SortedPlaylist {}

impl Playlist for SortedPlaylist {}