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
    total_discs: Option<u16>
}

impl Album {
    /// Creates a new [`Album`] with the given values.
    pub fn new(
        name: String,
        artist: Weak<Artist>,
        tracks: Vec<Arc<Track>>,
        year: Option<i32>,
        total_tracks: Option<u16>,
        total_discs: Option<u16>
    ) -> Album {
        Album {
            name,
            artist,
            tracks,
            year,
            total_tracks,
            total_discs
        }
    }

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

    /// Returns the [`Album`]'s total number of discs.
    pub fn total_discs(&self) -> &Option<u16> {
        &self.total_discs
    }

    /// Returns the [`Album`]'s duration by taking the sum of all [`Track`]s' durations.
    pub fn duration(&self) -> f64 {
        self.tracks.iter().map(|t| t.duration()).sum()
    }

    /// Returns the number of [`Track`]s in the [`Album`].
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Appends the provided [`Track`] to this [`Album`]'s tracks.
    pub fn push_track(&mut self, track: Arc<Track>) {
        self.tracks.push(track);
    }
}

impl Playable for Album {}

impl Shuffleable for Album {}