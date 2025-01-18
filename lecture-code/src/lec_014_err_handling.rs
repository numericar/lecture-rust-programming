pub fn process() {
    let result = divide(10.0, 0.0);

    match &result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e)
    }

    let val = result.unwrap();
    println!("{}", val);
    
}

fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        return Err(String::from("Cannot divide by zero"))
    } else {
        return Ok(a / b)
    }
}