use std::{fmt::{self, Write, Debug, Display}, sync::Arc};

use super::{executable::Executable, queueable::Queueable};

#[derive(Debug, Clone)]
pub struct Queued {
    index: usize,
    items: Vec<Arc<dyn Executable>>,
    _src: Arc<dyn Queueable>,
}

impl Queued {
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

impl Display for Queued {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queued (index: {}) [\n{}]", self.index,
            self.items.iter().map(|a| a.name()).fold(String::new(), |mut output, b| {
                let _ = writeln!(output, "  {}", b.to_string().replace('\n', "\n  "));
                output
            })
        )
    }
}

impl<T> From<Arc<T>> for Queued where T: Queueable + 'static {
    fn from(value: Arc<T>) -> Self {
        Queued {
            index: 0,
            items: value.executables(),
            _src: value,
        }
    }
}

impl From<Arc<dyn Queueable>> for Queued {
    fn from(value: Arc<dyn Queueable>) -> Self {
        Queued {
            index: 0,
            items: value.executables(),
            _src: value,
        }
    }
}

impl From<Arc<dyn Executable>> for Queued {
    fn from(value: Arc<dyn Executable>) -> Self {
        Queued {
            index: 0,
            items: vec![value.clone()],
            _src: value,
        }
    }
}