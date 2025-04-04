use std::{collections::{btree_map::Entry, BTreeMap, BTreeSet}, fmt::{Display, Write}, path::PathBuf, sync::Arc};

use audiotags::Tag;
use derive_more::derive::{Display, Error};
use glob::glob;

use crate::{media::{Artist, Album, Track}, playlist::Playlist};

// Library holds ownership the whole way down.
#[derive(Debug)]
pub struct Library {
    name: String,
    path: PathBuf,
    artists: BTreeSet<Arc<Artist>>,
    playlists: Vec<Arc<Playlist>>
    // At the moment, playlists are managed by a Library, I'll probably change that in the future
}

impl Library {
    /// Returns the [`Library`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Library`]'s [`Artist`]s.
    pub fn artists(&self) -> &BTreeSet<Arc<Artist>> {
        &self.artists
    }

    /// Returns the [`Library`]'s [`Playlist`]s.
    pub fn playlists(&self) -> &Vec<Arc<Playlist>> {
        &self.playlists
    }

    pub fn from_path(name: String, dir: String) -> Result<Library, LibraryReadErr> {
        // FIXME: passing Strings by value
        let info = read_library(dir.clone())?;
        Ok(Library {
            name,
            path: PathBuf::from(dir.clone()),
            artists: info.artists,
            playlists: read_playlists(dir, &info.path_to_track)
        })
    }
}

// impl Queueable for Library {
//     fn executables(&self) -> Vec<Arc<dyn Executable>> {
//         self.artists().iter().flat_map(|t| t.executables()).collect()
//     }
// } // ? Does this make sense

// impl Shuffleable for Library {}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.artists().iter().fold(String::new(), |mut output, b| {
            let _ = writeln!(output, "{}", b);
            output
        }))
    }
}

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

pub struct LibraryReadInfo {
    artists: BTreeSet<Arc<Artist>>,
    pub path_to_track: BTreeMap<PathBuf, Arc<Track>>
    // TODO: need to provide several maps for playlist creation
}

pub fn read_library(dir: String) -> Result<LibraryReadInfo, LibraryReadErr> {
    let dir_glob = dir + "/*/*/*.mp3";
    let tags = glob(&dir_glob).or(Err(FileReadErr::Pattern))?
        .filter_map(|file| file.ok()).map(|file| Ok((
            Tag::new().read_from_path(&file).or(Err(FileReadErr::Tag))?,
            file
        ))).collect::<Result<Vec<_>, FileReadErr>>()?;

    let mut path_to_track: BTreeMap<PathBuf, Arc<Track>> = BTreeMap::new();
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
                    BTreeSet::new()
                ));
                artist_entry.insert(new_artist);
            }

            let artist = artists.get_mut(&tag_artist).unwrap();

            let new_album = Arc::new(Album::new(
                tag_album.title.to_owned(),
                Arc::downgrade(artist),
                BTreeSet::new(),
                tag.year().or_else(|| tag.date().map(|d| d.year)),
                tag.total_tracks(),
                tag.total_discs()
            ));

            unsafe {
                Arc::get_mut_unchecked(artist).insert_album(new_album.clone())
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
            Arc::get_mut_unchecked(album).insert_track(track.clone())
        };

        path_to_track.insert(path.to_owned(), track);
    }
    // println!("{:?}", tracks);
    // println!("{:?}", albums.values().collect::<BTreeSet<_>>());
    // println!("{:?}", artists.values().collect::<BTreeSet<_>>());

    Ok(LibraryReadInfo {
        artists: artists.into_values().collect(),
        path_to_track
    })
}

pub fn read_playlists(_dir: String, _path_to_tracks: &BTreeMap<PathBuf, Arc<Track>>) -> Vec<Arc<Playlist>> {
    /*
        TODO
        Find all playlist files (or load them from memory)
        Read all and create a list
     */
    Vec::new()
}