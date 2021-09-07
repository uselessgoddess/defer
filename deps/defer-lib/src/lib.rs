#![feature(fn_traits)]
#![feature(box_syntax)]

pub struct Defer {
    at_exit: Vec<Box<dyn FnMut()>>
}

impl Defer {
    pub fn new() -> Self {
        Self { at_exit: vec![] }
    }

    pub fn push(&mut self, callback: Box<dyn FnMut()>) {
        self.at_exit.push(callback)
    }
}

impl Drop for Defer {
    fn drop(&mut self) {
        for callback in (&mut self.at_exit).into_iter().rev() {
            callback.call_mut(())
        }
    }
}

mod tests {
    use crate::Defer;

    #[test]
    fn test() {
        let mut defer = Defer::new();
        defer.push(box || { println!("1") });
        defer.push(box || { println!("2") });
        defer.push(box || { println!("3") });
    }
}