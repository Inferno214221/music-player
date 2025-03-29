use std::{any::Any, collections::{BTreeMap, BTreeSet}, fs, path::{Path, PathBuf}, sync::Arc};

use derive_more::derive::{Display, Error};

use crate::{library::track::Track, queue::{executable::Executable, queueable::Queueable, shuffleable::Shuffleable}};

#[derive(Debug)]
pub struct Playlist {
    name: String,
    p_type: PlaylistType
}

#[derive(Debug)]
pub enum PlaylistType {
    Linear {
        items: Vec<Arc<dyn Queueable>>
    },
    Sorted {
        items: BTreeSet<Arc<dyn Queueable>>
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

    pub fn items(&self) -> Box<dyn Iterator<Item = &Arc<dyn Queueable>> + '_> {
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
                            path_to_tracks.get(&track_path).map(|t| t.clone() as Arc<dyn Queueable>)
                        })
                        .collect() // TODO: need to think this one out, should a sorted playlist store more than tracks??
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
                            path_to_tracks.get(&track_path).map(|t| t.clone() as Arc<dyn Queueable>)
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