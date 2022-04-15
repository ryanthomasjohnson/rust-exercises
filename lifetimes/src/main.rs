// TODO: let the compiler know that never reading `S`'s fields is ok.

const Y: &str = "y";

// TODO: add lifetime specifier
struct S {
    x: &str,
    // TODO: what is `static` here? Where does it come from?
    y: &'static str,
}

fn main() {
    // TODO: why isn't this valid? What's one way to fix it?
    let s = {
        let x = String::from("x");
        S {
            x: &x,
            y: Y,
        }
    };
    // TODO: get this print to compile
    println!("{:?}", s);
}
