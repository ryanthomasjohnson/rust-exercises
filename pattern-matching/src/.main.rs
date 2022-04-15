fn evaluate(x: Result<Option<u8>, String>) -> Result<u8, String> {
    match x {
        Ok(x) => match x {
            None => Ok(0),
            Some(x) if x <= 10 => Ok(x),
            Some(x) => Err(format!("`{}` is invalid!", x)),
        },
        Err(err) => Err(err),
    }
}

fn trim_starting_and_ending_0(x: &[u32]) -> Result<&[u32], ()> {
    match x {
        [0, middle @ .., 0] => Ok(middle),
        [0] => Ok(&[]),
        _ => Err(()),
    }
}

fn main() {
    for x in [
        Ok(Some(5)),
        Ok(Some(100)),
        Ok(None),
        Err(String::from("this is an error")),
    ] {
        println!("evaluate({:?}) -> {:?}", x.clone(), evaluate(x));
    }
    let x = vec![0, 1, 2, 3, 0];
    println!("{:?} without 0's is {:?}", x, trim_starting_and_ending_0(&x));
    let x = vec![1, 2, 3];
    println!("{:?} without 0's is {:?}", x, trim_starting_and_ending_0(&x));
    let x = vec![0];
    println!("{:?} without 0's is {:?}", x, trim_starting_and_ending_0(&x));
}
