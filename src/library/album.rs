use std::sync::{Arc, Weak};

use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::{artist::Artist, track::Track};

#[derive(Debug)]
pub struct Album {
    name: String,
    artist: Weak<Artist>,
    tracks: Vec<Arc<Track>>, // ? Should this be a set of some type
    year: Option<i32>,
    total_tracks: Option<u16>,
    total_disks: Option<u16>
}

impl Album {
    /// Returns the [`Album`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a weak arc to the [`Album`]'s [`Artist`].
    pub fn album(&self) -> Weak<Artist> {
        // Should this just return a weak ref?
        self.artist.clone()
    }

    /// Returns the [`Album`]'s [`Track`]s.
    pub fn tracks(&self) -> &Vec<Arc<Track>> {
        &self.tracks
    }

    /// Returns the [`Album`]'s release year.
    pub fn year(&self) -> &Option<i32> {
        &self.year
    }

    /// Returns the [`Album`]'s total number of tracks (not the number available).
    pub fn total_tracks(&self) -> &Option<u16> {
        &self.total_tracks
    }

    /// Returns the [`Album`]'s total number of disks.
    pub fn total_disks(&self) -> &Option<u16> {
        &self.total_disks
    }

    /// Returns the [`Album`]'s duration by taking the sum of all [`Track`]s' durations.
    pub fn duration(&self) -> Option<f64> {
        self.tracks.iter().map(|t| t.duration().as_ref()).sum()
    }

    /// Returns the number of [`Track`]s in the [`Album`].
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

impl Playable for Album {}

impl Shuffleable for Album {}