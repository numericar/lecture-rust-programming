trait Weapon {
    fn attack(&self);
}

struct Sword;
struct Staff;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Attact with sword")
    }
}

impl Weapon for Staff {
    fn attack(&self) {
        println!("Attact with staff")
    }
}


struct Warrior {
    health: i16,
    intelligent: i16,
    strength: i16,
    weapon: Box<dyn Weapon>
}

struct Mage {
    health: i16,
    intelligent: i16,
    strength: i16,
    weapon: Box<dyn Weapon>
}

struct Healer {
    health: i16,
    intelligent: i16,
    strength: i16,
    weapon: Box<dyn Weapon>
}

impl Warrior {
    // initial instance of struct
    fn new() -> Self {
        Self {
            health: 100,
            strength: 10,
            intelligent: 10,
            weapon: Box::new(Sword)
        }
    }

    fn health_increse(&mut self, val: i16) {
        if self.health + val > 100 {
            self.health = 100;
            return;
        }

        self.health += val;
    }

    fn health_decrease(&mut self, val: i16) {
        self.health = self.health.saturating_sub(val);
    }
}

impl Mage {
    // initial instance of struct
    fn new() -> Self {
        Self {
            health: 100,
            strength: 5,
            intelligent: 15,
            weapon: Box::new(Staff)
        }
    }

    fn health_increse(&mut self, val: i16) {
        if self.health + val > 100 {
            self.health = 100;
            return;
        }

        self.health += val;
    }

    fn health_decrease(&mut self, val: i16) {
        self.health = self.health.saturating_sub(val);
    }
}

fn special_attact(weapon: Box<dyn Weapon>) {
    weapon.attack();
}

pub fn process() {
    let mut warr = Warrior::new();
    let mut mage: Mage = Mage::new();

    warr.health_decrease(10);
    mage.health_decrease(30);

    println!("{}", warr.health);
    println!("{}", mage.health);
}