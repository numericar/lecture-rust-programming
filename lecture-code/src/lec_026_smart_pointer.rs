use std::rc::Rc;
use std::cell::RefCell;

pub fn process() {
    statement_box();
    statement_rc();
    statement_ref_cell();
    statement_rc_refcell();
}

fn statement_box() -> () {
    let x = Box::new(10);
    println!("ค่าใน box: {}", x);
}

fn statement_rc() {
    let val = Rc::new(10);

    let a = Rc::clone(&val);
    let b = Rc::clone(&val);

    println!("A: {}", a);
    println!("B: {}", b);
    println!("Reference count: {}", Rc::strong_count(&val)); // ใช้รับ ownership ว่ามีใครกำลังใช้อยู่บ้าง
}

fn statement_ref_cell() {
    let data = RefCell::new(5);

    println!("ค่า่เริ่มต้น: {}", data.borrow());

    *data.borrow_mut() += 10;

    println!("หลังดำเนินการ: {}", data.borrow());
}

fn statement_rc_refcell() {
    let shared_data = Rc::new(RefCell::new(10));

    let data_clone1 = Rc::clone(&shared_data);
    let data_clone2 = Rc::clone(&shared_data);

    *data_clone1.borrow_mut() += 10;

    println!("data clone 1: {}", data_clone1.borrow());
    println!("data clone 2: {}", data_clone2.borrow());
}