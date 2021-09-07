extern crate defer_macro;
extern crate defer_lib;

pub use defer_lib::{*};
pub use defer_macro::{*};

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    #[use_defer]
    fn it_works() {
        println!("enter");
        defer! { println!("exit"); }
        return;

        defer! { println!("after exit"); }
    }

    #[test]
    #[use_defer]
    fn error_in_naive_implementation() {
        for i in 0..10 {
            defer! { println!("{}", i) }
        }
        println!("done")
    }
}
