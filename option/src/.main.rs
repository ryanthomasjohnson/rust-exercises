fn maybe_double(x: Option<u32>) -> Option<u32> {
    x.map(|x| x * 2)
}

fn double_or_0(x: Option<u32>) -> u32 {
    x.map_or(0, |x| x * 2)
}

fn filter_out_none(x: Vec<Option<u32>>) -> Vec<u32> {
    // Option implements IntoIterator!
    x.into_iter().flatten().collect()
}

fn main() {
    println!("Double {:?} is {:?}", Some(4), maybe_double(Some(4)));
    let x = None;
    println!("Double {:?} is {:?}", x, maybe_double(x));
    println!("Double {:?} or 0 is {}", Some(4), double_or_0(Some(4)));
    println!("Double {:?} or 0 is {}", None::<u32>, double_or_0(None));
    let x = vec![Some(1), None, Some(2), None];
    println!("{:?} filtered is {:?}", x.clone(), filter_out_none(x));
}
