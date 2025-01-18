// classic struct
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool
}

// tuple struct 
struct Position(i32, i32, i32);

pub fn process() {
    let user: User = User {
        username: "Rosetta".to_string(),
        email: "mint@gmail.com".to_string(),
        sign_in_count: 10,
        active: true
    };
    println!("{} {} {} {}", user.username, user.email, user.sign_in_count, user.active);

    let user_position: Position = Position(100,39,4);
    println!("x: {}, y: {}, z: {}", user_position.0, user_position.1, user_position.2);
}