#![feature(linked_list_cursors)]
#![feature(get_mut_unchecked)]

#![allow(clippy::module_inception)] // TODO: better module names


use library::library::Library;
use soloud::Soloud;

pub mod library;
pub mod playlist;
pub mod queue;

fn main() {
    let l = Library::from_path(
            String::from("Main"),
            String::from("/home/inferno214221/Music")
    ).unwrap();
    // dbg!(&l);
    println!("{}", l);

    // let sl = Soloud::default().unwrap();

    // let mut q = Queue::new();
    // let a = l.artists().first().unwrap().clone().as_queueable();
    // q.add_next(Arc::downgrade(&a));
    // q.play(&sl);
}
