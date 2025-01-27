#![feature(linked_list_cursors)]
#![allow(clippy::module_inception)] // TODO: better module names

use que::que::Que;

pub mod library;
pub mod playlist;
pub mod que;

fn main() {
    let mut q = Que::new();
    let c = q.cursor();
    dbg!(c.current().as_ref());
    c.move_next();
    dbg!(c.current().as_ref());
    c.move_next();
    dbg!(c.current().as_ref());
    c.move_prev();
    dbg!(c.current().as_ref());
}
