use std::{collections::{linked_list::CursorMut, LinkedList}, sync::Weak};

use super::queueable::Queueable;

pub type QueueCursor<'a> = CursorMut<'a, Weak<dyn Queueable>>;

#[derive(Debug)]
pub struct Queue<'a> {
    items: LinkedList<Weak<dyn Queueable>>,
    cursor: Option<QueueCursor<'a>>
}

impl<'a> Queue<'a> {
    /// Creates an empty [`Queue`]. (No items and a lazy cursor.)
    pub fn new() -> Queue<'a> {
        Queue {
            items: LinkedList::new(),
            cursor: None
        }
    }

    /// Returns a mutable reference to the [`Queue`]'s items.
    pub fn items(&'a mut self) -> &'a mut LinkedList<Weak<dyn Queueable>> {
        &mut self.items
    }

    /// Returns a mutable reference to the [`Queue`]'s cursor. The cursor is created now if it hasn't
    /// been already.
    pub fn cursor(&'a mut self) -> &'a mut QueueCursor<'a> {
        if self.cursor.is_none() {
            self.cursor = Some(self.items.cursor_front_mut());
        }
        self.cursor.as_mut().unwrap()
    }

    /// Inserts the provided [`Queueable`] into the [`Queue`] after the current item.
    pub fn queue_next(&'a mut self, queueable: Weak<dyn Queueable>) {
        self.cursor().insert_after(queueable);
    }

    /// Appends the provided [`Queueable`] to the end of the [`Queue`].
    pub fn queue_end(&'a mut self, queueable: Weak<dyn Queueable>) {
        self.items().push_back(queueable);
    }
}

impl Default for Queue<'_> {
    fn default() -> Self {
        Self::new()
    }
}