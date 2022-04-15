struct OddError(pub u32);

// TODO: implement Display for OddError

fn half_of(x: u32) -> Result<u32, OddError> {
    // TODO: return half of x if x is even, otherwise return OddError
}

fn main() {
    for x in 0..4 {
        match half_of(x) {
            Ok(y) => println!("Half of {} is {}", x, y),
            Err(err) => println!("Error: {}", err),
        }
    }
}
