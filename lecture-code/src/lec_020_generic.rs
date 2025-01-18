pub fn process() {
    print_two_val("10", "20");

    let position_int: Pointer<i32> = Pointer { x: 10, y: 20 };
    println!("{} {}", position_int.x, position_int.y);
}

fn print_two_val<T: std::fmt::Display>(value1: T, value2: T) {
    println!("1: {}", value1);
    println!("2: {}", value2);
}

// สร้าง struct ร่วมกับ generic
struct Pointer<T> {
    x: T,
    y: T
}