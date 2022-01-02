#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

extern crate defer_lib;
extern crate defer_macro;

pub use defer_lib::{*};
pub use defer_macro::{*};

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;
    use super::*;

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

    #[test]
    fn before_panic() {
        catch_unwind(#[use_defer] || {
            defer! { println!("defer before panic"); }
            println!("before panic");
            panic!("panic");
        }).unwrap_err();
    }

    #[test]
    fn closure() {
        let closure = #[use_defer] || { defer! { println!("closure") } };
        closure();
    }
}
