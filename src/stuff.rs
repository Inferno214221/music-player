use std::{collections::{linked_list::CursorMut, BTreeSet, LinkedList}, fmt::Debug, sync::{Arc, Weak}};

pub trait Playable: Debug {}

#[derive(Debug)]
pub struct Track {
    name: String,
    album: Weak<Album>,
    duration: Option<f64>, // ? Should this really be optional
    track_number: Option<u16>,
    disc_number: Option<u16>
}

impl Track {}

impl Playable for Track {}

pub trait Shuffleable: Debug {}

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
    /// Returns the album's duration by taking the sum of all tracks' durations.
    pub fn duration(&self) -> Option<f64> {
        self.tracks.iter().map(|t| t.duration).sum()
    }

    /// Returns the number of tracks in the album.
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

impl Playable for Album {}

impl Shuffleable for Album {}

#[derive(Debug)]
pub struct Artist {
    name: String,
    albums: Vec<Arc<Album>> // ? Should this be a set of some type
}

impl Artist {}

impl Playable for Artist {} // ? Does this make sense

impl Shuffleable for Artist {}

pub trait Playlist: Debug {}

#[derive(Debug)]
pub struct LinearPlaylist {
    name: String,
    items: Vec<Weak<dyn Playable>>
}

impl LinearPlaylist {}

impl Playable for LinearPlaylist {}

impl Shuffleable for LinearPlaylist {}

impl Playlist for LinearPlaylist {}

#[derive(Debug)]
pub struct SortedPlaylist {
    name: String,
    items: BTreeSet<Weak<dyn Playable>> // Sorting passes through the Arc
}

impl SortedPlaylist {}

impl Playable for SortedPlaylist {} // ? Does this make sense

impl Shuffleable for SortedPlaylist {}

impl Playlist for SortedPlaylist {}

#[derive(Debug)]
// Library holds ownership the whole way down.
pub struct Library {
    artists: Vec<Arc<Artist>>, // ? Should this be a set of some type
    playlists: Vec<Weak<dyn Playlist>>
}

impl Library {}

impl Playable for Library {} // ? Does this make sense

impl Shuffleable for Library {}

pub trait Queable: Debug {}

impl Queable for dyn Playable {}

impl<T> Queable for T where T: Playable {}

impl Queable for dyn Shuffleable {}

// TODO: impl<T> Queable for T where T: Shuffleable {}

#[derive(Debug)]
pub struct QuePause;

impl Queable for QuePause {}

#[derive(Debug)]
pub struct QueStop;

impl Queable for QueStop {}

pub type QueCursor<'a> = CursorMut<'a, Weak<dyn Queable>>;

#[derive(Debug)]
pub struct Que<'a> {
    items: LinkedList<Weak<dyn Queable>>,
    cursor: Option<QueCursor<'a>>
}

impl<'a> Que<'a> {
    /// Creates an empty [`Que`]. (No items and a lazy cursor.)
    pub fn new() -> Que<'a> {
        Que {
            items: LinkedList::new(),
            cursor: None
        }
    }

    /// Returns a mutable reference to the [`Que`]'s items.
    pub fn items(&'a mut self) -> &'a mut LinkedList<Weak<dyn Queable>> {
        &mut self.items
    }

    /// Returns a mutable reference to the [`Que`]'s cursor. The cursor is created now if it hasn't
    /// been already.
    pub fn cursor(&'a mut self) -> &'a mut QueCursor<'a> {
        if self.cursor.is_none() {
            self.cursor = Some(self.items.cursor_front_mut());
        }
        self.cursor.as_mut().unwrap()
    }
}

impl Default for Que<'_> {
    fn default() -> Self {
        Self::new()
    }
}