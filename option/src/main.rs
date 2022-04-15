fn maybe_double(x: Option<u32>) -> Option<u32> {
    // TODO: double x if it is not None
}

fn double_or_0(x: Option<u32>) -> u32 {
    // TODO: double x if it is not None or return 0
}

fn filter_out_none(x: Vec<Option<u32>>) -> Vec<u32> {
    // TODO: Remove `None`s from x
}

fn main() {
    println!("Double {:?} is {:?}", Some(4), maybe_double(Some(4)));
    // TODO: get this next line to compile (try creating a variable)
    println!("Double {:?} is {:?}", None, maybe_double(None));
    println!("Double {:?} or 0 is {}", Some(4), double_or_0(Some(4)));
    // TODO: get this next line to compile (try turbofish)
    println!("Double {:?} or 0 is {}", None, double_or_0(None));
    let x = vec![Some(1), None, Some(2), None];
    println!("{:?} filtered is {:?}", x.clone(), filter_out_none(x));
}
