use std::sync::{Arc, Weak};

use crate::playlist::playlist::Playlist;
use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::artist::Artist;

// Library holds ownership the whole way down.
#[derive(Debug)]
pub struct Library {
    name: String,
    artists: Vec<Arc<Artist>>, // ? Should this be a set of some type
    playlists: Vec<Weak<dyn Playlist>>
}

impl Library {
    /// Returns the [`Library`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Library`]'s [`Artist`]s.
    pub fn albums(&self) -> &Vec<Arc<Artist>> {
        &self.artists
    }

    /// Returns the [`Library`]'s [`Playlist`]s.
    pub fn playlists(&self) -> &Vec<Weak<dyn Playlist>> {
        &self.playlists
    }
}

impl Playable for Library {} // ? Does this make sense

impl Shuffleable for Library {}