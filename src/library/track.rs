use std::{path::{Path, PathBuf}, sync::Weak};

use crate::que::playable::Playable;

use super::album::Album;

#[derive(Debug)]
pub struct Track {
    name: String,
    path: PathBuf,
    album: Weak<Album>,
    track_number: Option<u16>,
    disc_number: Option<u16>
}

impl Track {
    /// Creates a new [`Track`] with the given values.
    pub fn new(
        name: String,
        path: PathBuf,
        album: Weak<Album>,
        track_number: Option<u16>,
        disc_number: Option<u16>
    ) -> Track {
        Track {
            name,
            path,
            album,
            track_number,
            disc_number
        }
    }
    
    /// Returns the [`Track`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Track`]'s path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns a weak arc to the [`Track`]'s [`Album`].
    pub fn album(&self) -> Weak<Album> {
        // Should this just return a weak ref?
        self.album.clone()
    }

    /// Returns the [`Track`]'s track number (of the album).
    pub fn track_number(&self) -> &Option<u16> {
        &self.track_number
    }

    /// Returns the [`Track`]'s disc number.
    pub fn disc_number(&self) -> &Option<u16> {
        &self.disc_number
    }

    /// Returns the [`Track`]'s duration.
    pub fn duration(&self) -> &f64 {
        todo!()
    }
}

impl Playable for Track {}