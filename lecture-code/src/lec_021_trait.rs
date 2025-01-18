pub fn process() {
    let person: Person = Person {
        name: String::from("Alice")
    };
    person.say_hello();

    let nums = Numbers(vec![1, 2, 3, 4, 5]);
    print_sum(nums);
}

struct Person {
    name: String
}

trait Greet {
    fn say_hello(&self);
}

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

trait Summable {
    fn sum(&self) -> i32;
}

struct Numbers(Vec<i32>);

impl Summable for Numbers {
    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
}

fn print_sum<T: Summable>(item: T) {
    println!("The sum is: {}", item.sum());
}