use std::collections::HashMap;

pub fn process() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 40);
    scores.insert("Bike", 90);
    println!("{:?}", scores);

    // การเข้าถึงข้อมูล
    if let Some(score) = scores.get("Alice") {
        println!("Alice score: {}", score);
    } else {
        println!("Alice not found");
    }

    for (name, score) in &scores {
        println!("{} {}", name, score);
    }

    // การแก้ไขข้อมูล
    scores.insert("Alice", 60); // แก้ไขข้อมูลของ alice

    scores.entry("David").or_insert(90); // ใช้ entry api ฟังก์ชัน or_insert โดยจะ insert ก็ต่อเมื่อไม่มี key
    println!("David score: {}", scores.get("David").unwrap());

    // ลบข้อมูล
    scores.remove("Alice");
    println!("{:?}", scores);
}