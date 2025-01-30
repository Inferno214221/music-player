use std::{cmp::Ordering, collections::BTreeSet, fmt::{self, Display, Formatter, Write}, sync::Arc};

use crate::queue::{playable::Playable, shuffleable::Shuffleable};

use super::album::Album;

#[derive(Debug)]
pub struct Artist {
    name: String,
    albums: BTreeSet<Arc<Album>>
}

impl Artist {
    /// Creates a new [`Artist`] with the provided values.
    pub fn new(
        name: String,
        albums: BTreeSet<Arc<Album>>
    ) -> Artist {
        Artist {
            name,
            albums
        }
    }

    /// Returns the [`Artist`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Artist`]'s [`Album`]s.
    pub fn albums(&self) -> &BTreeSet<Arc<Album>> {
        &self.albums
    }

    /// Inserts the provided [`Album`] into this [`Artist`]'s albums.
    pub fn insert_album(&mut self, album: Arc<Album>) {
        self.albums.insert(album);
    }
}

impl Playable for Artist {}

impl Shuffleable for Artist {}

impl Display for Artist {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [\n{}]",
            self.name,
            self.albums.iter().fold(String::new(), |mut output, b| {
                let _ = writeln!(output, "  {}", b.to_string().replace('\n', "\n  "));
                output
            })
        )
    }
}

impl PartialEq for Artist {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // Exclude albums to prevent recursion
    }
}

impl Eq for Artist {}

impl PartialOrd for Artist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Artist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}