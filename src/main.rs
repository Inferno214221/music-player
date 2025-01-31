#![feature(linked_list_cursors)]
#![feature(get_mut_unchecked)]

#![allow(clippy::module_inception)] // TODO: better module names

use core::time;
use std::{thread, sync::{Arc, Weak}};

use library::library::Library;
use queue::{player::Player, queue::Queue, queue_operations::QueuePause, queueable::Queueable, shuffleable::Shuffleable};

pub mod library;
pub mod playlist;
pub mod queue;

fn main() {
    let l = Library::from_path(
            String::from("Main"),
            String::from("/home/inferno214221/Music")
    ).unwrap();
    // println!("{}", &l);

    let s = Arc::new(l.shuffled());
    // println!("{}", l.shuffled());

    let mut q = Queue::new();
    let pause = Arc::new(QueuePause);
    q.add_next(Arc::downgrade(&s) as Weak<dyn Queueable>);
    q.add_next(Arc::downgrade(&pause) as Weak<dyn Queueable>);

    // let (mut manager, _backend) = awedio::start().unwrap();
    // let (sound, mut controller) = q.exec(None).unwrap().unwrap();
    // manager.play(Box::new(sound));
    // thread::sleep(time::Duration::from_secs(5));
    // controller.set_paused(true);
    // thread::sleep(time::Duration::from_secs(5));
    // controller.set_paused(false);
    // thread::sleep(time::Duration::from_secs(5));
    // manager.clear();
    // let (sound_1, mut controller) = q.exec(Some(controller)).unwrap().unwrap();
    // manager.play(Box::new(sound_1));
    // thread::sleep(time::Duration::from_secs(30));

    let _ = q.play();
    // let _ = q.play();
    thread::sleep(time::Duration::from_secs(10));
}
