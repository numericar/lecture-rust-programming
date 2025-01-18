pub fn process() {
    // สร้าง String ใหม่
    let mut greeting: String = String::new();
    greeting.push_str("Hello");
    println!("{}", greeting);

    // สร้าง String จาก literal string
    let mut name = String::from("bike");

    // การเข้าถึงข้อมูล
    println!("{}", name); // เข้าถึงข้อมูลทั้งหมด
    println!("{}", name.chars().next().unwrap()); // การเข้าถึงข้อความตัวแรก
    println!("{}", name.chars().last().unwrap()); // การเข้าถึงข้อความตัวท้าย
    println!("{}", &name.chars().nth(1).unwrap()); // เข้าถึงข้อความจากตำแหน่งที่ระบุ

    // การเชื่อมต่อข้อความ
    let additional_text: &str = " Hello!";
    name.push_str(additional_text);
    println!("{}", name);

    // การลบข้อความ
    let mut replace_name = name.replace("Hello", "World");
    println!("{}", replace_name);
    println!("{}", name);

    replace_name.pop(); // ลบอักขระตัวสุดท้ายออก
    println!("{}", replace_name);
}