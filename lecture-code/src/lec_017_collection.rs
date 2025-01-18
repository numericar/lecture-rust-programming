pub fn process() {
    lec_vector();
}

fn lec_vector() -> () {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    println!("{:?}", numbers);

    let mut fruits: Vec<&str> = vec!["apple", "banana"];

    // เพิ่มข้อมูล
    fruits.push("mongo");

    // เข้าถึงข้อมูล
    println!("{}", fruits[0]);

    // การเข้าถึงข้อมูลแบบปลอดภัย
    match fruits.get(5) {
        Some(val) => println!("ข้อมูลในตำแหน่งที่ 5: {}", val),
        None => println!("ไม่มีข้อมูลในตำแหน่งที่ 5")
    }

    // เข้าถึงข้อมูลแบบวนซ้่ำ
    for fruit in &fruits {
        println!("{}", *fruit);
    }

    // ปรับเปลี่ยนข้อมูล
    if let Some(fruit) = fruits.get_mut(0) {
        *fruit = "Malago";
    }

    // การลบข้อมูล
    fruits.pop(); // ลบข้อมูลคำแหน่งสุดท้าย   
    fruits.remove(0); // ลบข้อมูลตามตำแหน่งที่ระบุ

    println!("{:?}", fruits);
}