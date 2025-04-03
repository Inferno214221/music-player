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
    FormatError,
    PathError
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

    pub fn from_file(path: &Path, path_to_tracks: &BTreeMap<PathBuf, Arc<Track>>)
        -> Result<Playlist, PlaylistParseErr> {
        // Check type (Linear as default)
        let file_contents = fs::read_to_string(path).or(Err(ReadError))?;
        // Regex match for sorted playlist
        let sorted = true;
        let items = file_contents.split('\n')
            .filter(|l| !l.starts_with('#'))
            // TODO: store and read more than tracks
            .filter_map(|l| {
                // TODO: notify about failed gets
                let mut track_path = PathBuf::from(l);
                if !track_path.is_absolute() {
                    // TODO: if it starts with nothing, assume its relative to the library
                    track_path = PathBuf::from(
                        path.parent().expect("Playlist has a parent directory")
                    );
                    track_path.push(PathBuf::from(l));
                }
                path_to_tracks.get(
                    &fs::canonicalize(track_path).ok()? // TODO: report on this too
                ).map(|t| t.clone() as Arc<dyn Playlistable>)
            });

        Ok(Playlist {
            name: path.file_prefix()
                .ok_or(FormatError)?
                .to_str()
                .ok_or(FormatError)?
                .to_owned(), // OR read attribute
            p_type: if sorted {
                Sorted {
                    items: items.collect()
                }
            } else {
                Linear {
                    items: items.collect()
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