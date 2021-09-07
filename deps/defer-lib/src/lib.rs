use std::collections::LinkedList;

type DeferContainer<T> = LinkedList<T>;

pub struct Defer {
    at_exit: DeferContainer<Box<dyn FnMut()>>
}

impl Defer {
    pub fn new() -> Self {
        Self { at_exit: Default::default() }
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
    use crate::Defer;

    #[test]
    fn test() {
        let mut defer = Defer::new();
        defer.push(Box::new(|| { println!("1") }));
        defer.push(Box::new(|| { println!("2") }));
        defer.push(Box::new(|| { println!("3") }));
    }
}