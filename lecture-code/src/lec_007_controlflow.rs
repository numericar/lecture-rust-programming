pub fn process() {
    if_else_statement();
    match_statement();
}

fn if_else_statement() {
    let number = 7;

    if number < 5 {
        println!("Small");
    } else if number == 5 {
        println!("Medium");
    } else {
        println!("Large");
    }
}

fn match_statement() {
    let number = 2;

    // match แบบปกติ
    match number {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("Failed")
    }

    // match แบบมีตัวแปรมารับค่า
    let result = match number {
        1 => 100,
        2 => 200,
        _ => 0
    };

    println!("{}", result);
}