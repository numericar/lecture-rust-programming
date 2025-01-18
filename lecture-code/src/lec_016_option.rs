pub fn process() {
    let result = find_rquare_root(-8.0);

    match result {
        Some(val) => println!("Sqrt: {}", val),
        None => println!("Error")
    }
}

fn find_rquare_root(number: f32) -> Option<f32> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}