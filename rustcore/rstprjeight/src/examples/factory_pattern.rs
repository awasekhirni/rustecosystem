//factory pattern

trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

enum AnimalType {
    Dog,
    Cat,
}

struct AnimalFactory;

impl AnimalFactory {
    fn create_animal(animal_type: AnimalType) -> Box<dyn Animal> {
        match animal_type {
            AnimalType::Dog => Box::new(Dog),
            AnimalType::Cat => Box::new(Cat),
        }
    }
}

// let dog = AnimalFactory::create_animal(AnimalType::Dog);
// dog.speak();

// let cat = AnimalFactory::create_animal(AnimalType::Cat);
// cat.speak();
