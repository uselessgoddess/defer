#![feature(box_syntax)]
#![feature(default_free_fn)]

use std::default::default;
use std::collections::LinkedList;

type DeferContainer<T> = LinkedList<T>;

pub struct Defer {
    at_exit: DeferContainer<Box<dyn FnMut()>>
}

impl Defer {
    pub fn new() -> Self {
        Self { at_exit: default() }
    }

    pub fn push(&mut self, callback: Box<dyn FnMut()>) {
        self.at_exit.push_back(callback)
    }
}

impl Drop for Defer {
    fn drop(&mut self) {
        for callback in (&mut self.at_exit).into_iter().rev() {
            (callback)()
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut defer = Defer::new();
        defer.push(box || { println!("1") });
        defer.push(box || { println!("2") });
        defer.push(box || { println!("3") });
    }
}