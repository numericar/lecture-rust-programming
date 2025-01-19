pub fn process() {
    let robot = Robot;
    let human = Human;

    greet(&robot);
    greet(&human);
}

trait Greet {
    fn say_hello(&self);
}

struct Robot;
struct Human;

impl Greet for Robot {
    fn say_hello(&self) {
        println!("Greeting, human! I am robot.");
    }
}

impl  Greet for Human {
    fn say_hello(&self) {
        println!("Greeting, robot! I'm a human.");
    }
}

fn greet(greeter: &dyn Greet) {
    greeter.say_hello();
}