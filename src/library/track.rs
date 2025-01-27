use std::sync::Weak;

use crate::que::playable::Playable;

use super::album::Album;

#[derive(Debug)]
pub struct Track {
    name: String,
    album: Weak<Album>,
    duration: Option<f64>, // ? Should this really be optional
    track_number: Option<u16>,
    disc_number: Option<u16>
}

impl Track {
    /// Returns the [`Track`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a weak arc to the [`Track`]'s [`Album`].
    pub fn album(&self) -> Weak<Album> {
        // Should this just return a weak ref?
        self.album.clone()
    }

    /// Returns the [`Track`]'s duration.
    pub fn duration(&self) -> &Option<f64> {
        &self.duration
    }

    /// Returns the [`Track`]'s track number (of the album).
    pub fn track_number(&self) -> &Option<u16> {
        &self.track_number
    }

    /// Returns the [`Track`]'s disc number.
    pub fn disc_number(&self) -> &Option<u16> {
        &self.disc_number
    }
}

impl Playable for Track {}