use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};

use audiotags::Tag;
use derive_more::derive::{Display, Error};
use glob::glob;

use crate::playlist::playlist::Playlist;
use crate::que::{playable::Playable, shuffleable::Shuffleable};

use super::album::Album;
use super::artist::Artist;
use super::track::Track;

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

    pub fn from_path(name: String, dir: String) -> Result<Library, LibraryReadErr> {
        Ok(Library {
            name,
            artists: read_library(dir.clone())?,
            playlists: read_playlists(dir)
        })
    }
}

impl Playable for Library {} // ? Does this make sense

impl Shuffleable for Library {}

#[derive(Debug, Display, Error, PartialEq)]
pub enum MissingMetaErr {
    Title,
    Album,
    Artist
}

#[derive(Debug, Display, Error, PartialEq)]
pub enum FileReadErr {
    Pattern, // TODO: glob mightn't be the best way to find files anyway
    Tag
}

#[derive(Debug, Display, Error, PartialEq)]
pub enum LibraryReadErr {
    Meta(MissingMetaErr),
    File(FileReadErr)
}

impl From<MissingMetaErr> for LibraryReadErr {
    fn from(value: MissingMetaErr) -> Self {
        LibraryReadErr::Meta(value)
    }
}

impl From<FileReadErr> for LibraryReadErr {
    fn from(value: FileReadErr) -> Self {
        LibraryReadErr::File(value)
    }
}

pub fn read_library(dir: String) -> Result<Vec<Arc<Artist>>, LibraryReadErr> {
    let dir_glob = dir + "/*/*/*.mp3";
    let tags = glob(&dir_glob).or(Err(FileReadErr::Pattern))?
        .filter_map(|file| file.ok()).map(|file| Ok((
            Tag::new().read_from_path(&file).or(Err(FileReadErr::Tag))?,
            file
        ))).collect::<Result<Vec<_>, FileReadErr>>()?;

    let mut tracks = Vec::new();
    let mut albums: BTreeMap<(&str, &str), Arc<Album>> = BTreeMap::new();
    let mut artists: BTreeMap<&str, Arc<Artist>> = BTreeMap::new();
    for (tag, path) in tags.iter() {
        // println!("{:?}", path);
        let tag_album: audiotags::types::Album = tag.album().ok_or(MissingMetaErr::Album)?;
        let tag_artist = tag.artist().ok_or(MissingMetaErr::Artist)?;
        let key = (tag_album.title, tag_artist);

        if let Entry::Vacant(album_entry) = albums.entry(key) {
            if let Entry::Vacant(artist_entry) = artists.entry(tag_artist) {
                let new_artist = Arc::new(Artist::new(
                    tag_artist.to_owned(),
                    Vec::new()
                ));
                artist_entry.insert(new_artist);
            }

            let artist = artists.get_mut(&tag_artist).unwrap();

            let new_album = Arc::new(Album::new(
                tag_album.title.to_owned(),
                Arc::downgrade(artist),
                Vec::new(),
                tag.year(),
                tag.total_tracks(),
                tag.total_discs()
            ));

            unsafe {
                Arc::get_mut_unchecked(artist).push_album(new_album.clone())
            };

            album_entry.insert(new_album);
        }

        let album = albums.get_mut(&key).unwrap();

        let track = Arc::new(Track::new(
            tag.title().ok_or(MissingMetaErr::Title)?.to_owned(),
            path.clone(),
            Arc::downgrade(album),
            tag.track_number(),
            tag.disc_number()
        ));

        // TODO: I'm pretty sure this is safe, but it still feels off.
        // Only done for partial initialisation with two way references.
        unsafe {
            Arc::get_mut_unchecked(album).push_track(track.clone())
        };

        tracks.push(track);
    };
    // println!("{:?}", tracks);
    // println!("{:?}", albums.values().collect::<Vec<_>>());
    // println!("{:?}", artists.values().collect::<Vec<_>>());

    Ok(artists.into_values().collect())
}

pub fn read_playlists(_dir: String) -> Vec<Weak<dyn Playlist>> {
    // TODO
    Vec::new()
}