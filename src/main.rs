#![feature(get_mut_unchecked)]
#![feature(path_file_prefix)]

#![allow(clippy::module_inception)]

pub mod controller;
pub mod media;
pub mod playlist;
pub mod queue;

use std::{path::PathBuf, sync::Arc};

use controller::{read_library, Library, Queue};
use playlist::Playlist;

fn main() {
    let l = Library::from_path(
        "Main".into(),
        "/home/inferno214221/music/library".into()
    ).unwrap();

    let mut q = Queue::new();

    let rhcp = l.artists().iter().find(|a| a.name() == "Red Hot Chili Peppers").unwrap().clone();
    q.add_end(
        rhcp.albums().iter().find(|a| a.name() == "The Getaway").unwrap()
            .tracks().iter().find(|a| a.name() == "This Ticonderoga").unwrap().clone()
    );
    q.add_end(
        rhcp.albums().iter().find(|a| a.name() == "Californication").unwrap().clone()
    );
    q.add_end(
        rhcp.albums().iter().find(|a| a.name() == "By the Way").unwrap()
            .tracks().iter().find(|a| a.name() == "Venice Queen").unwrap().clone()
    );
    for _ in 1..14 {
        let _ = q.skip();
    }
    println!("{}", &q);
    println!("{}", &q.current().unwrap().current().unwrap());
    q.decompose(1);
    println!("{}", &q);
    println!("{}", &q.current().unwrap().current().unwrap());

    let p = Arc::new(Playlist::from_file(
        PathBuf::from(String::from("/home/inferno214221/music/playlists/new.m3u")).as_path(),
        &read_library("/home/inferno214221/music/library".into()).unwrap().path_to_track
    ).unwrap());
    q.add_end(p.clone());
    println!("{q}");

    // let _ = q.play();
    // thread::sleep(Duration::from_secs(10));
    // q.pause();
    //
    // thread::sleep(Duration::from_secs(60));
}