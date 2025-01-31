use std::sync::Weak;

use super::{executable::PlayError, player::Player, queueable::Queueable};

#[derive(Debug)]
pub struct Queue {
    items: Vec<Weak<dyn Queueable>>,
    index: usize,
    player: Player
}

impl Queue {
    /// Creates an empty [`Queue`]. (No items and a lazy cursor.)
    pub fn new() -> Queue {
        Queue {
            items: Vec::new(),
            index: 0,
            player: Player::new().unwrap()
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
        let new_index = match self.current() {
            Some(_) => self.index + 1,
            None => self.index
        };
        self.items.insert(new_index, queueable);
    }

    /// Appends the provided [`Queueable`] to the end of the [`Queue`].
    pub fn add_end(&mut self, queueable: Weak<dyn Queueable>) {
        self.items().push(queueable);
    }

    pub fn play(&mut self) -> Result<(), PlayError> {
        self.current().unwrap()
            .upgrade().unwrap()
            .executables().first().unwrap()
            .exec(&mut self.player)
    }

    pub fn skip(&mut self) -> Result<(), PlayError> {
        self.player.manager().clear();
        self.index += 1;
        self.play()
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}