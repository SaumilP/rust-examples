struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Static method signature
    // `Self` refers to the interpretor type
    fn new(name: &'static str) -> Self;

    // Instance method signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits - which can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name());
            self.naked = true;
        }
    }
}

// Implementat the `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaah!!!"
        }
    }

    // default trait method can be overriden in implementation
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name(), self.noise());
    }
}

fn main() {
    // type annotation is necessary for this case
    let mut sam:Sheep = Animal::new("Sam");

    sam.talk();
    sam.shear();
    sam.talk();
}
