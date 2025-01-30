use std::sync::Weak;

use super::queueable::Queueable;

#[derive(Debug)]
pub struct Queue {
    items: Vec<Weak<dyn Queueable>>,
    index: usize
}

impl Queue {
    /// Creates an empty [`Queue`]. (No items and a lazy cursor.)
    pub fn new() -> Queue {
        Queue {
            items: Vec::new(),
            index: 0
        }
    }

    /// Returns a mutable reference to the [`Queue`]'s items.
    pub fn items(&mut self) -> &mut Vec<Weak<dyn Queueable>> {
        &mut self.items
    }

    /// Returns a mutable reference to the [`Queue`]'s cursor. The cursor is created now if it
    /// hasn't been already.
    pub fn current(&mut self) -> Option<&Weak<dyn Queueable>> {
        self.items.get(self.index)
    }

    /// Inserts the provided [`Queueable`] into the [`Queue`] after the current item.
    pub fn add_next(&mut self, queueable: Weak<dyn Queueable>) { // TODO: should own with Arc
        self.items.insert(self.index + 1, queueable);
    }

    /// Appends the provided [`Queueable`] to the end of the [`Queue`].
    pub fn add_end(&mut self, queueable: Weak<dyn Queueable>) {
        self.items().push(queueable);
    }

    // pub fn play(&mut self, sl: &Soloud) -> Option<()> {
    //     let current_item = self.cursor().current().and_then(|w| w.upgrade().clone())?;
    //     current_item.exec(sl);
    //     None
    // }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}