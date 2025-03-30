use std::{collections::{BTreeMap, BTreeSet}, fs, path::{Path, PathBuf}, sync::Arc};

use derive_more::derive::{Display, Error};

use crate::{media::Track, queue::{Executable, Queueable, Shuffleable}};

use super::playlistable::Playlistable;

#[derive(Debug)]
pub struct Playlist {
    name: String,
    p_type: PlaylistType
}

#[derive(Debug)]
pub enum PlaylistType {
    Linear {
        items: Vec<Arc<dyn Playlistable>>
    },
    Sorted {
        items: BTreeSet<Arc<dyn Playlistable>>
    }
}

use PlaylistType::*;

#[derive(Debug, Display, Error)]
pub enum PlaylistParseErr {
    ReadError,
    FormatError
}

use PlaylistParseErr::*;

impl Playlist {
    /// Returns the [`Playlist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn items(&self) -> Box<dyn Iterator<Item = &Arc<dyn Playlistable>> + '_> {
        match &self.p_type {
            Linear{items} => Box::new(items.into_iter()),
            Sorted{items} => Box::new(items.into_iter())
        }
    }

    pub fn read(path: &Path, path_to_tracks: &BTreeMap<PathBuf, Arc<Track>>)
        -> Result<Playlist, PlaylistParseErr> {
        // Check type (Linear as default)
        let file_contents = fs::read_to_string(path).or(Err(ReadError))?;
        // Regex match for sorted playlist
        let sorted = true;
        Ok(Playlist {
            name: path.file_prefix()
                .ok_or(FormatError)?
                .to_str()
                .ok_or(FormatError)?
                .to_owned(), // OR read attribute
            p_type: if sorted {
                Sorted {
                    items: file_contents
                        .split('\n')
                        .filter(|l| !l.starts_with('#'))
                        .filter_map(|l| {
                            let mut track_path = PathBuf::from(l);
                            if !track_path.is_absolute() {
                                track_path = PathBuf::from(
                                    path.parent().expect("Playlist has a parent directory")
                                );
                                track_path.push(track_path.clone());
                            }
                            // TODO: store and read more than tracks
                            // TODO: notify about failed gets
                            path_to_tracks.get(&track_path).map(|t| t.clone() as Arc<dyn Playlistable>)
                        })
                        .collect()
                }
            } else {
                Linear {
                    items: file_contents
                        .split('\n')
                        .filter(|l| !l.starts_with('#'))
                        .filter_map(|l| {
                            let mut track_path = PathBuf::from(l);
                            if !track_path.is_absolute() {
                                track_path = PathBuf::from(
                                    path.parent().expect("Playlist has a parent directory")
                                );
                                track_path.push(track_path.clone());
                            }
                            // TODO: store and read more than tracks
                            // TODO: notify about failed gets
                            path_to_tracks.get(&track_path).map(|t| t.clone() as Arc<dyn Playlistable>)
                        })
                        .collect()
                }
            }
        })
    }
}

impl Queueable for Playlist {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        self.items().flat_map(|i| i.executables()).collect()
    }
}

impl Shuffleable for Playlist {}