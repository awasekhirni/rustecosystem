//Composition Over Inheritance

struct Engine {
    horsepower: u32,
}

pub struct Car {
    engine: Engine,
    model: String,
}

impl Car {
    pub fn new(model: String, horsepower: u32) -> Self {
        Car {
            engine: Engine { horsepower },
            model,
        }
    }

    pub fn specs(&self) -> String {
        format!("Model: {}, HP: {}", self.model, self.engine.horsepower)
    }
}

// let my_car = Car::new("Tesla Model 3".to_string(), 283);
// println!("{}", my_car.specs());
