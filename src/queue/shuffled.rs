use std::{any::Any, fmt::{self, Display, Formatter, Write}, sync::Arc};

use rand::rng;
use rand::seq::SliceRandom;

use super::{executable::Executable, queueable::Queueable, shuffleable::Shuffleable};

#[derive(Debug)]
pub struct Shuffled {
    // TODO: reference source as well?
    items: Vec<Arc<dyn Executable>>
}

impl Shuffled {}

impl Queueable for Shuffled {
    fn executables(&self) -> Vec<Arc<dyn Executable>> {
        self.items.clone() // TODO: I think this is wrong too.
    }
}

impl<T> From<&T> for Shuffled where T: Shuffleable {
    fn from(value: &T) -> Self {
        let mut items = value.executables().clone();
        items.shuffle(&mut rng());
        Shuffled {
            items
        }
    }
}

impl Display for Shuffled {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\n{}]",
            self.items.iter().fold(String::new(), |mut output, b| {
                let _ = writeln!(output, "  {}", b);
                output
            })
        )
    }
}