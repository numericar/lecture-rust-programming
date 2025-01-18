enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32) 
}

pub fn process() {
    let msg = Message::Write("Hello World".to_string());
    print_message(msg);
}

fn print_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit Message"),
        Message::Move {x, y} => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g , b ) => println!("Change color: {} {} {}", r, g, b),
    }
}