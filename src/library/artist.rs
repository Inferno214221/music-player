use std::sync::Arc;

use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::album::Album;

#[derive(Debug)]
pub struct Artist {
    name: String,
    albums: Vec<Arc<Album>> // ? Should this be a set of some type
}

impl Artist {
    /// Creates a new [`Artist`] with the given values.
    pub fn new(
        name: String,
        albums: Vec<Arc<Album>>
    ) -> Artist {
        Artist {
            name,
            albums
        }
    }

    /// Returns the [`Artist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Artist`]'s [`Album`]s.
    pub fn albums(&self) -> &Vec<Arc<Album>> {
        &self.albums
    }

    /// Appends the provided [`Album`] to this [`Artist`]'s tracks.
    pub fn push_album(&mut self, album: Arc<Album>) {
        self.albums.push(album);
    }
}

impl Playable for Artist {} // ? Does this make sense

impl Shuffleable for Artist {}