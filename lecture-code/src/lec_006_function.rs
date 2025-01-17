pub fn process() -> () {
    let result: u8 = add(5, 5);
    println!("{}", result);
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}