use std::{cmp::Ordering, fmt::Debug};

use crate::{media::{Album, Artist, Track}, queue::Queueable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlaylistItemType<'a> {
    // Sort playlist items by type first
    Artist(&'a Artist),
    Album(&'a Album),
    Track(&'a Track)
}

pub trait Playlistable: Queueable + Debug + Send + Sync {
    fn as_item_type(&self) -> PlaylistItemType;
}

impl PartialEq for dyn Playlistable {
    fn eq(&self, other: &Self) -> bool {
        self.as_item_type() == other.as_item_type()
    }
}

impl Eq for dyn Playlistable {}

impl PartialOrd for dyn Playlistable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn Playlistable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_item_type().cmp(&other.as_item_type())
    }
}