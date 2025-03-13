#![feature(linked_list_cursors)]
#![feature(get_mut_unchecked)]

#![allow(clippy::module_inception)] // TODO: better module names

use core::time;
use std::{sync::Arc, thread};
use time::Duration;
use library::library::Library;
use queue::{queue::Queue, shuffleable::Shuffleable};

pub mod library;
pub mod playlist;
pub mod queue;

fn main() {
    let l = Library::from_path(
        "Main".into(),
        "/home/inferno214221/Music".into()
    ).unwrap();

    let mut q = Queue::new();
    q.add_next(Arc::new(l.shuffled()));
    // q.add_next(Arc::new(QueuePause));
    dbg!(&q);

    let _ = q.play();
    thread::sleep(Duration::from_secs(10));
    q.pause();

    thread::sleep(Duration::from_secs(60));
}