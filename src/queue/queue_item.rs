use std::fmt::Debug;
use std::sync::Arc;
use crate::queue::executable::Executable;
use crate::queue::queueable::Queueable;

#[derive(Debug)]
pub struct QueueItem {
    index: usize,
    items: Vec<Arc<dyn Executable>>,
    _src: Arc<dyn Queueable>,
}

impl QueueItem {
    pub fn current(&mut self) -> Option<&mut Arc<dyn Executable>> {
        self.items.get_mut(self.index)
    }

    pub fn skip(&mut self) -> Option<&mut Arc<dyn Executable>> {
        self.index += 1;
        self.current()
    }

    pub fn prev(&mut self) -> Option<&mut Arc<dyn Executable>> {
        self.index -= 1;
        self.current()
    }
}

impl<T> From<Arc<T>> for QueueItem where T: Queueable + 'static {
    fn from(value: Arc<T>) -> Self {
        QueueItem {
            index: 0,
            items: value.executables(),
            _src: value,
        }
    }
}

impl From<Arc<dyn Queueable>> for QueueItem {
    fn from(value: Arc<dyn Queueable>) -> Self {
        QueueItem {
            index: 0,
            items: value.executables(),
            _src: value,
        }
    }
}