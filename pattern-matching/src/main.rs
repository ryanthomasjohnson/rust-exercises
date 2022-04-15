fn evaluate(x: Result<Option<u8>, String>) -> Result<u8, String> {
    // TODO: implement this with a `match` statement:
    //  - if x is an error, return the provided error.
    //  - if x is None, return 0.
    //  - if x is <= 10, return x
    //  - if x is > 10, return an Error (see expected output for the message)
}

fn trim_starting_and_ending_0(x: &[u32]) -> Result<&[u32], ()> {
    // TODO: if x starts with and ends with 0, return a slice without it/them.
}

fn main() {
    for rating in [
        Ok(Some(5)),
        Ok(Some(100)),
        Ok(None),
        Err(String::from("this is an error")),
    ] {
        // TODO: get this print to compile.
        println!("evaluate({:?}) -> {:?}", rating, evaluate(rating));
    }
    let x = vec![0, 1, 2, 3, 0];
    println!("{:?} without 0's is {:?}", x, trim_starting_and_ending_0(&x));
    let x = vec![1, 2, 3];
    println!("{:?} without 0's is {:?}", x, trim_starting_and_ending_0(&x));
}
