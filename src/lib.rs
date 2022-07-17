pub use defer_proc::*;

use std::collections::LinkedList;

#[derive(Default)]
pub struct Defer<'a> {
    defers: LinkedList<Box<dyn FnMut() + 'a>>,
}

impl<'a> Defer<'a> {
    pub fn new() -> Self {
        Self {
            defers: LinkedList::new(),
        }
    }

    pub fn push(&mut self, callback: impl FnMut() + 'a) {
        self.defers.push_back(Box::new(callback))
    }
}

impl Drop for Defer<'_> {
    fn drop(&mut self) {
        for callback in self.defers.iter_mut() {
            callback()
        }
    }
}
