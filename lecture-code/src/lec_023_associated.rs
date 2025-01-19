pub fn process() {
    let mut counter = Counter { count: 4 };

    while let Some(val) = counter.next() {
        println!("{}", val);
    }
}

trait Iteratorx {
    type Item;

    fn next(&mut self) -> Option<Self::Item>; 
}

struct Counter {
    count: u32
}

impl Iteratorx for Counter {
    type Item = u32;

    // next จะ return เป็น option<u32> ตามที่กำหนดไว้ใน item
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}