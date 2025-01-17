pub fn process() {
    let mut s1 = String::from("Hello");

    // ยืมค่าจาก s1 ส่งให้กับ calculate_length ทำงาน
    let len = calculate_length(&s1);

    format_string(&mut s1);

    println!("{}", len);
    println!("{}", s1);
}

// ยืมแบบอ่านอย่างเดียว
fn calculate_length(s: &String) -> usize {
    s.len()
}

// ยืมแบบอ่านและแก้ไข
fn format_string(s: &mut String) -> () {
    *s = "Rosetta".to_string();
}