// Define a base trait for common behavior
trait Animal {
    fn speak(&self);
    fn move_(&self);
}

// Define a struct for a Dog
struct Dog {
    name: String,
}

impl Dog {
    // Constructor method for Dog
    fn new(name: &str) -> Self {
        Dog {
            name: name.to_string(),
        }
    }
}

// Implement the Animal trait for Dog (polymorphism via trait)
impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name);
    }

    fn move_(&self) {
        println!("{} runs around!", self.name);
    }
}

// Define a struct for a Cat
struct Cat {
    name: String,
}

impl Cat {
    // Constructor method for Cat
    fn new(name: &str) -> Self {
        Cat {
            name: name.to_string(),
        }
    }
}

// Implement the Animal trait for Cat (polymorphism via trait)
impl Animal for Cat {
    fn speak(&self) {
        println!("{} says Meow!", self.name);
    }

    fn move_(&self) {
        println!("{} jumps gracefully!", self.name);
    }
}

// Demonstrating polymorphism via trait objects
fn animal_action(animal: &dyn Animal) {
    animal.speak();
    animal.move_();
}

fn main() {
    // Creating instances of Dog and Cat
    let dog = Dog::new("Buddy");
    let cat = Cat::new("Whiskers");

    // Demonstrating polymorphism: treating both Dog and Cat as Animal
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    for animal in animals {
        animal_action(animal);
    }
}
