pub fn process() {
    let s1 = String::from("Hello");
    let s2 = s1; //  ค่าจาก s1 ถูกย้ายไปยัง s2

    // println!("S1: {}", s1); // ไม่สามารถใช้งานได้อีกต่อไป เนื่องจากค่าใน s1 ถูกเปลี่ยนเจ้าของเป็น s2 แล้ว, s1 จะถูก drop
    println!("S2: {}", s2);
}