pub fn process() {
    statement_debug(10);
    statement_clone(String::from("Rust"));
    statement_partial_eq(20 , 10);
    statment_ord(1, 2);
}

// ใช้ trait debug เพื่อแสดงค่าในรูปแบบของการ Debug (่ผ่าน "{:?}")
fn statement_debug<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val);
}

// ใช้ trait Clone เพื่อทำการคัดลอก object 
fn statement_clone<T: Clone>(val: T) -> (T, T) {
    (val.clone(), val)
}

// ใช้ trait PartialEq เพื่อบอกว่า ข้อมูลที่จะส่งเข้ามา จะต้องมีการ impl trait PartrialEq หมายถึง ให้ใช้ == ได้่
fn statement_partial_eq<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}

// ใช้ trait Ord เพื่อจัดการกับค่าที่จะต้องเปรียบเทียบมากกว่าหรือน้อยกว่ากันได้
fn statment_ord<T: Ord>(a: T, b: T) -> &'static str {
    if a < b {
        "a is less than b"
    } else if a > b {
        "a is greater than b"
    } else {
        "a is equal b"
    }
}