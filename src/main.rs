#![feature(linked_list_cursors)]
#![feature(get_mut_unchecked)]

#![allow(clippy::module_inception)] // TODO: better module names

use library::library::Library;

pub mod library;
pub mod playlist;
pub mod queue;

fn main() {
    let l = Library::from_path(
            String::from("Main"),
            String::from("/home/inferno214221/Music")
    ).unwrap();
    println!("{:?}", l);
}
