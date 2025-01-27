#![feature(linked_list_cursors)]
#![allow(clippy::module_inception)] // TODO: better module names
#![feature(get_mut_unchecked)]

use read_files::read_library;

pub mod library;
pub mod playlist;
pub mod que;
pub mod read_files;

fn main() {
    // let mut q = Que::new();
    // let c = q.cursor();
    // dbg!(c.current().as_ref());
    // c.move_next();
    // dbg!(c.current().as_ref());
    // c.move_next();
    // dbg!(c.current().as_ref());
    // c.move_prev();
    // dbg!(c.current().as_ref());
    let _ = read_library(String::from("/home/inferno214221/Music"));
}
