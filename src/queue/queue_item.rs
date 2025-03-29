use super::{executable::Executable, queueable::Queueable};
use std::fmt::{self, Write, Debug, Display};
use std::sync::Arc;

#[derive(Debug, Clone)]
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
        if self.index >= self.items.len() {
            self.index -= 1;
            None
        } else {
            self.current()
        }
    }

    pub fn prev(&mut self) -> Option<&mut Arc<dyn Executable>> {
        self.index -= 1;
        // FIXME: possible usize issue
        self.current()
    }

    pub fn executables(&self) -> &Vec<Arc<dyn Executable>> {
        &self.items
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl Display for QueueItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QueueItem (index: {}) [\n{}]", self.index,
            self.items.iter().map(|a| a.name()).fold(String::new(), |mut output, b| {
                let _ = writeln!(output, "  {}", b.to_string().replace('\n', "\n  "));
                output
            })
        )
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

impl From<Arc<dyn Executable>> for QueueItem {
    fn from(value: Arc<dyn Executable>) -> Self {
        QueueItem {
            index: 0,
            items: vec![value.clone()],
            _src: value,
        }
    }
}