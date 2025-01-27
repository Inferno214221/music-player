use std::{collections::{linked_list::CursorMut, LinkedList}, sync::Weak};

use super::queable::Queable;

pub type QueCursor<'a> = CursorMut<'a, Weak<dyn Queable>>;

#[derive(Debug)]
pub struct Que<'a> {
    items: LinkedList<Weak<dyn Queable>>,
    cursor: Option<QueCursor<'a>>
}

impl<'a> Que<'a> {
    /// Creates an empty [`Que`]. (No items and a lazy cursor.)
    pub fn new() -> Que<'a> {
        Que {
            items: LinkedList::new(),
            cursor: None
        }
    }

    /// Returns a mutable reference to the [`Que`]'s items.
    pub fn items(&'a mut self) -> &'a mut LinkedList<Weak<dyn Queable>> {
        &mut self.items
    }

    /// Returns a mutable reference to the [`Que`]'s cursor. The cursor is created now if it hasn't
    /// been already.
    pub fn cursor(&'a mut self) -> &'a mut QueCursor<'a> {
        if self.cursor.is_none() {
            self.cursor = Some(self.items.cursor_front_mut());
        }
        self.cursor.as_mut().unwrap()
    }
}

impl Default for Que<'_> {
    fn default() -> Self {
        Self::new()
    }
}