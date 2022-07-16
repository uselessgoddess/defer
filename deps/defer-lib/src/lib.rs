use std::collections::LinkedList;

#[derive(Default)]
pub struct Defer {
    defers: LinkedList<Box<dyn FnMut()>>,
}

impl Defer {
    pub fn new() -> Self {
        Self {
            defers: LinkedList::new(),
        }
    }

    pub fn push(&mut self, callback: Box<dyn FnMut()>) {
        self.defers.push_back(callback)
    }
}

impl Drop for Defer {
    fn drop(&mut self) {
        for callback in self.defers.iter_mut() {
            callback()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Defer;

    #[test]
    fn test() {
        let mut defer = Defer::new();
        defer.push(Box::new(|| println!("1")));
        defer.push(Box::new(|| println!("2")));
        defer.push(Box::new(|| println!("3")));
    }
}
