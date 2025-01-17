pub fn process() {
    // Tuple
    let tup: (i32, f64, char) = (400, 2.2, 'a');
    let (x, y, z) = tup; // สามารถ Destructuring ได้
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    for i in v.iter() {
        println!("{}", i);
    }
}