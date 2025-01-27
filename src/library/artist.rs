use std::sync::Arc;

use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::album::Album;

#[derive(Debug)]
pub struct Artist {
    name: String,
    albums: Vec<Arc<Album>> // ? Should this be a set of some type
}

impl Artist {
    /// Returns the [`Artist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Artist`]'s [`Album`]s.
    pub fn albums(&self) -> &Vec<Arc<Album>> {
        &self.albums
    }
}

impl Playable for Artist {} // ? Does this make sense

impl Shuffleable for Artist {}