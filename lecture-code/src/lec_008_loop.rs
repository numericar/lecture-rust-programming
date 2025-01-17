pub fn process() {
    loop_statement();
    while_statement();
    for_statement();
}

fn loop_statement() {
    let mut count: u8 = 0;
    
    loop {
        count += 1;

        if count >= 5 {
            break;
        }
    }

    println!("Loop finished");
}

fn while_statement() {
    let mut number = 3;

    while number != 0 {
        number -= 1;
    }

    println!("While finished");
}

fn for_statement() {
    for number in 1..4 {
        println!("{}", number);
    }
    println!("While finished");
}