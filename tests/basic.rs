// for better experience
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

use defer::{defer, use_defer};

#[test]
#[use_defer]
fn it_works() {
    println!("enter");
    defer! { println!("exit"); }
    return;

    #[allow(unreachable_code)]
    {
        defer! { println!("after exit"); }
    }
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
    let closure = #[use_defer]
    || {
        defer! { println!("defer after panic"); }
        println!("before panic");
        panic!("panic");
    };
    assert!(matches!(std::panic::catch_unwind(closure), Err(_)));
}

#[test]
fn closure() {
    let closure = #[use_defer]
    || {
        defer! { println!("closure") }
    };
    closure();
}

#[test]
#[use_defer]
fn use_borrowed_values() {
    let mut v = 0;
    let weak = &v as *const i32;
    defer! {
        // SAFETY: Not guaranteed, other destructors may contain ref to `v`,
        // but in this example it is safe
        println!("defer: {}", unsafe { *weak });
    }
    v = 1;
    println!("home: {}", v);
}
