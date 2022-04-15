#![allow(dead_code)]

const Y: &str = "y";

#[derive(Debug)]
struct S<'a> {
    x: &'a str,
    y: &'static str,
}

fn main() {
    let x = String::from("x");
    let s = S {
        x: &x,
        y: Y,
    };
    println!("{:?}", s);
}
