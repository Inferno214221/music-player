use std::{collections::{btree_map::Entry, BTreeMap}, sync::Arc};

use audiotags::Tag;
use derive_more::derive::{Display, Error};
use glob::glob;

use crate::library::{album::Album, track::Track};

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

pub fn read_library(dir: String) -> Result<(), LibraryReadErr> {
    let dir_glob = dir + "/*/*/*.mp3";
    let tags = glob(&dir_glob).or(Err(FileReadErr::Pattern))?
        .filter_map(|file| file.ok()).map(|file| Ok((
            Tag::new().read_from_path(&file).or(Err(FileReadErr::Tag))?,
            file
        ))).collect::<Result<Vec<_>, FileReadErr>>()?;

    let mut tracks = Vec::new();
    let mut albums: BTreeMap<(&str, Option<&str>), Arc<Album>> = BTreeMap::new();
    for (tag, path) in tags.iter() {
        // println!("{:?}", path);
        let tag_album = tag.album().ok_or(MissingMetaErr::Album)?;
        let key = (tag_album.title, tag_album.artist);

        if let Entry::Vacant(entry) = albums.entry(key) { 
            let new_album = Arc::new(Album::new(
                tag_album.title.to_owned(),
                None,
                Vec::new(),
                tag.year(),
                tag.total_tracks(),
                tag.total_discs()
            ));
            entry.insert(new_album);
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
        unsafe { Arc::get_mut_unchecked(album).push_track(track.clone()) };

        tracks.push(track);
    };
    println!("{:?}", tracks);
    println!("{:?}", albums.values().collect::<Vec<_>>());

    Ok(())
}