use std::sync::Arc;
use crate::queue::queue_item::QueueItem;
use super::{executable::PlayError, player::Player, queueable::Queueable};

#[derive(Debug)]
pub struct Queue {
    items: Vec<QueueItem>,
    index: usize,
    player: Player
}

impl Queue {
    /// Creates an empty [`Queue`].
    pub fn new() -> Queue {
        Queue {
            items: Vec::new(),
            index: 0,
            player: Player::new().unwrap()
        }
    }

    /// Returns a mutable reference to the [`Queue`]'s items.
    pub fn items(&mut self) -> &mut Vec<QueueItem> {
        &mut self.items
    }

    /// Returns a mutable reference to the [`Queue`]'s cursor. The cursor is created now if it
    /// hasn't been already.
    pub fn current(&mut self) -> Option<&mut QueueItem> {
        self.items.get_mut(self.index)
    }

    /// Inserts the provided [`QueueItem`] into the [`Queue`] after the current item.
    pub fn add_next(&mut self, queueable: Arc<dyn Queueable>) {
        // TODO: need two _next_ functions, one which decomposes the current queued item.
        let new_index = match self.current() {
            Some(_) => self.index + 1,
            None => self.index
        };
        self.items.insert(new_index, QueueItem::from(queueable));
    }

    /// Appends the provided [`QueueItem`] to the end of the [`Queue`].
    pub fn add_end(&mut self, queueable: Arc<dyn Queueable>) {
        self.items().push(QueueItem::from(queueable));
    }

    /// Plays the current executable.
    pub fn play(&mut self) -> Result<(), PlayError> {
        self.player.manager().clear();
        // self.items[self.index]
        self.current().unwrap()
            .current().unwrap()
            .clone().exec(&mut self.player) // TODO: should try to remove this second clone.
            // .exec(&mut self.player)
        // TODO: store timestamps.
    }

    /// Plays the next executable.
    pub fn skip(&mut self) -> Result<(), PlayError> {
        // There are currently two layers of lists nested, need to iterate the inner and then iterate the outer on overflow
        self.player.manager().clear();
        let next = self.current().unwrap().skip();
        if let Some(n) = next {
            n.clone().exec(&mut self.player)
        } else {
            self.index += 1;
            self.current().unwrap()
                .current().unwrap()
                .clone().exec(&mut self.player)
        }
    }

    /// Plays the previous executable.
    pub fn prev(&mut self) -> Result<(), PlayError> {
        todo!()
    }
    
    pub fn pause(&mut self) {
        if let Some(controller) = self.player.controller().as_mut() {
            controller.set_paused(true);
        }
    }
    
    pub fn resume(&mut self) {
        if let Some(controller) = self.player.controller().as_mut() {
            controller.set_paused(false);
        }
    }
    
    pub fn stop(&mut self) {
        self.player.manager().clear();
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}