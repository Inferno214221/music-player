use std::{fmt::{self, Write, Display}, sync::Arc};

use crate::queue::{PlayError, Playback, Queueable, Queued};

#[derive(Debug)]
pub struct Queue {
    items: Vec<Queued>,
    index: usize,
    player: Playback
}

impl Queue {
    /// Creates an empty [`Queue`].
    pub fn new() -> Queue {
        Queue {
            items: Vec::new(),
            index: 0,
            player: Playback::new().unwrap()
        }
    }

    /// Returns a mutable reference to the [`Queue`]'s items.
    pub fn items(&mut self) -> &mut Vec<Queued> {
        &mut self.items
    }

    /// Returns a mutable reference to the [`Queue`]'s cursor. The cursor is created now if it
    /// hasn't been already.
    pub fn current(&mut self) -> Option<&mut Queued> {
        self.items.get_mut(self.index)
    }

    pub fn add_next(&mut self, queueable: Arc<dyn Queueable>) {
        self.decompose(self.index);
        self.add_after(queueable);
    }

    /// Inserts the provided [`Queued`] into the [`Queue`] after the current item.
    pub fn add_after(&mut self, queueable: Arc<dyn Queueable>) {
        let new_index = match self.current() {
            Some(_) => self.index + 1,
            None => self.index
        };
        self.items.insert(new_index, Queued::from(queueable));
    }

    /// Appends the provided [`Queued`] to the end of the [`Queue`].
    pub fn add_end(&mut self, queueable: Arc<dyn Queueable>) {
        self.items.push(Queued::from(queueable));
    }

    /// Plays the current executable.
    pub fn play(&mut self) -> Result<(), PlayError> {
        self.player.manager().clear();
        // self.items[self.index]
        self.current().ok_or(PlayError::MissingItem)?
            .current().ok_or(PlayError::MissingItem)?
            .clone().exec(&mut self.player) // TODO: should try to remove this second clone.
            // .exec(&mut self.player)
        // TODO: store timestamps.
    }

    /// Plays the next executable.
    pub fn skip(&mut self) -> Result<(), PlayError> {
        // There are currently two layers of lists nested, need to iterate the inner and then iterate the outer on overflow
        self.player.manager().clear();
        let next = self.current().ok_or(PlayError::MissingItem)?.skip();
        if let Some(n) = next {
            n.clone().exec(&mut self.player)
        } else {
            self.index += 1;
            self.current().ok_or(PlayError::MissingItem)?
                .current().ok_or(PlayError::MissingItem)?
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

    pub fn decompose(&mut self, target_index: usize) {
        let item = self.items()[target_index].clone(); // should move?
        self.items.splice(
            target_index..(target_index + 1),
            item.executables().iter().map(
                |e| Queued::from(e.clone())
            )
        );

        if self.index >= target_index {
            self.index += item.index();
        }
    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Que (index: {}) [\n{}]", self.index,
            self.items.iter().map(|i| i.to_string()).fold(String::new(), |mut output, b| {
                let _ = writeln!(output, "  {}", b.to_string().replace('\n', "\n  "));
                output
            })
        )
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}