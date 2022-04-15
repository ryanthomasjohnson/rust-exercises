use std::fmt;

struct OddError(pub u32);

impl fmt::Display for OddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is odd!", self.0)
    }
}

fn half_of(x: u32) -> Result<u32, OddError> {
    if x % 2 == 0 {
        Ok(x / 2)
    } else {
        Err(OddError(x))
    }
}

fn main() {
    for x in 0..4 {
        match half_of(x) {
            Ok(y) => println!("Half of {} is {}", x, y),
            Err(err) => println!("Error: {}", err),
        }
    }
}
