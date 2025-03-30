use std::{cmp::Ordering, collections::BTreeSet, fmt::{self, Display, Formatter, Write}, sync::{Arc, Weak}};

use crate::{playlist::playlistable::{PlaylistItemType, Playlistable}, queue::{executable::Executable, queueable::Queueable, shuffleable::Shuffleable}};

use super::{artist::Artist, track::Track};

#[derive(Debug)]
pub struct Album {
    name: String,
    artist: Weak<Artist>,
    tracks: BTreeSet<Arc<Track>>,
    year: Option<i32>,
    total_tracks: Option<u16>,
    total_discs: Option<u16>
}

impl Album {
    /// Creates a new [`Album`] with the provided values.
    pub fn new(
        name: String,
        artist: Weak<Artist>,
        tracks: BTreeSet<Arc<Track>>,
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
    pub fn artist(&self) -> Weak<Artist> {
        // Should this just return a weak ref?
        self.artist.clone()
    }

    /// Returns the [`Album`]'s [`Track`]s.
    pub fn tracks(&self) -> &BTreeSet<Arc<Track>> {
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

    /// Inserts the provided [`Track`] into this [`Album`]'s tracks.
    pub fn insert_track(&mut self, track: Arc<Track>) {
        self.tracks.insert(track);
    }
}

impl Queueable for Album {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        self.tracks().iter().flat_map(|t| t.executables()).collect()
    }
}

impl Shuffleable for Album {}

impl Playlistable for Album {
    fn as_item_type(&self) -> PlaylistItemType {
        PlaylistItemType::Album(self)
    }
}

impl Display for Album {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}/{}) - {} ({}) [\n{}]",
            self.name,
            self.track_count(),
            self.total_tracks
                .map(|t| t.to_string())
                .unwrap_or(String::from("?")),
            self.artist.upgrade()
                .map(|a| a.name().to_owned())
                .unwrap_or(String::from("Unknown")),
            match self.year {
                Some(year) => year.to_string(),
                None => String::from("Unknown")
            },
            self.tracks.iter().fold(String::new(), |mut output, b| {
                let _ = writeln!(output, "  {}", b);
                output
            })
        )
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.year == other.year &&
        self.total_tracks == other.total_tracks &&
        self.artist.upgrade() == other.artist.upgrade()
        // Exclude tracks to prevent recursion
    }
}

impl Eq for Album {}

impl PartialOrd for Album {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Album {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.artist.upgrade().partial_cmp(&other.artist.upgrade()) {
            Some(Ordering::Equal) | None => (),
            Some(ord) => return ord
        }
        match self.year.partial_cmp(&other.year) {
            Some(Ordering::Equal) | None => (),
            Some(ord) => return ord
        }
        match self.name.cmp(&other.name) {
            Ordering::Equal => (),
            ord => return ord
        }
        self.total_tracks.cmp(&other.total_tracks)
    }
}