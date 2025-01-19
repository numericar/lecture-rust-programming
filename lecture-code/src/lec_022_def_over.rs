pub fn process() {
    let robot: Robot = Robot;

    robot.say_hello();
}

trait Greet {
    fn say_hello(&self) {
        println!("Hello!")
    }
}

struct Robot;

impl Greet for Robot {
    fn say_hello(&self) {
        println!("U II A U II A I");
    }
}

