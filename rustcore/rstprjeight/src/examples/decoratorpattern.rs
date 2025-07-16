//Decorator pattern

trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }

    fn description(&self) -> String {
        "Simple coffee".to_string()
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{} with milk", self.coffee.description())
    }
}

// let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
// println!("{}: ${}", coffee.description(), coffee.cost());

// let coffee_with_milk = MilkDecorator::new(coffee);
// println!("{}: ${}", coffee_with_milk.description(), coffee_with_milk.cost());
