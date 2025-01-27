#![feature(linked_list_cursors)]

use stuff::Que;

pub mod stuff;

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
