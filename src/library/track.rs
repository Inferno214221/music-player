use std::{cmp::Ordering, fmt::{self, Display, Formatter}, path::{Path, PathBuf}, sync::{Arc, Weak}};

use awedio::{sounds::{self}, Sound};

use crate::{playlist::playlistable::Playlistable, queue::{executable::{Executable, PlayError}, player::Player, queueable::Queueable}};

use super::album::Album;

#[derive(Debug, Clone)]
pub struct Track {
    name: String,
    path: PathBuf,
    album: Weak<Album>,
    track_number: Option<u16>,
    disc_number: Option<u16>
}

impl Track {
    /// Creates a new [`Track`] with the provided values.
    pub fn new(
        name: String,
        path: PathBuf,
        album: Weak<Album>,
        track_number: Option<u16>,
        disc_number: Option<u16>
    ) -> Track {
        Track {
            name,
            path,
            album,
            track_number,
            disc_number
        }
    }
    
    /// Returns the [`Track`]'s name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Track`]'s path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns a weak arc to the [`Track`]'s [`Album`].
    pub fn album(&self) -> Weak<Album> {
        // Should this just return a weak ref?
        self.album.clone()
    }

    /// Returns the [`Track`]'s track number (of the album).
    pub fn track_number(&self) -> &Option<u16> {
        &self.track_number
    }

    /// Returns the [`Track`]'s disc number.
    pub fn disc_number(&self) -> &Option<u16> {
        &self.disc_number
    }

    /// Returns the [`Track`]'s duration.
    pub fn duration(&self) -> &f64 {
        todo!()
    }
}

// impl Queueable for Track {
//     fn executables(&self) -> Vec<Arc<dyn Executable>> {
//         // FIXME: not sure that this is correct
//         vec![Arc::new(self.clone())]
//     }
// }

impl Queueable for Arc<Track> {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        vec![self.clone()]
    }
}

impl Executable for Track {
    fn exec(&self, player: &mut Player) -> Result<(), PlayError> {
        let (_sound, controller) = sounds::open_file(self.path())
            .or(Err(PlayError::FailedLoad))?
            .pausable()
            .controllable();
        println!("{:?}", self.path());
        *player.controller() = Some(controller);
        Ok(())
        // Ok(Some(
        //     sounds::open_file(self.path()).or(Err(PlayError::FailedLoad))?
        //         .pausable()
        //         .controllable()
        // ))

        // let mut wav = Wav::default();
        // dbg!(wav.load(dbg!(self.path())).or(Err(PlayError::FailedLoad))?);
        // let r = sl.play(&wav);
        // std::thread::sleep(std::time::Duration::from_millis((wav.length() * 1000_f64) as u64));
        // Ok(r)
    }
}

impl Playlistable for Track {}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} {} - {}",
            self.disc_number
                .map(|t| t.to_string())
                .unwrap_or(String::from("?")),
            self.track_number
                .map(|t| t.to_string())
                .unwrap_or(String::from("?")),
            self.name,
            self.album.upgrade()
                .map(|a| a.name().to_owned())
                .unwrap_or(String::from("Unknown"))
        )
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.path == other.path &&
        self.album.upgrade() == other.album.upgrade() &&
        self.track_number == other.track_number &&
        self.disc_number == other.disc_number
    }
}

impl Eq for Track {}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Track {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.album.upgrade().partial_cmp(&other.album.upgrade()) {
            Some(Ordering::Equal) | None => (),
            Some(ord) => return ord
        }
        match self.disc_number.partial_cmp(&other.disc_number) {
            Some(Ordering::Equal) | None => (),
            Some(ord) => return ord
        }
        match self.track_number.partial_cmp(&other.track_number) {
            Some(Ordering::Equal) | None => (),
            Some(ord) => return ord
        }
        match self.name.cmp(&other.name) {
            Ordering::Equal => (),
            ord => return ord
        }
        self.path.cmp(&other.path)
    }
}