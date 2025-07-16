mod examples;
use examples::*;

use crate::examples::{bankAccount::BankAccount, car::Car};

fn main() {
    println!("Project 8 Demo - Awase Khirni Syed ");
    println!("=== 1. Basic Struct with Methods ===");
    let mut account = BankAccount::new("Alice".to_string(), 100.0);
    account.deposit(50.0);
    println!("Balance: {}", account.get_balance());

    println!("\n=== 2. Traits for Polymorphism ===");
    let shapes: Vec<Box<dyn drawable::Drawable>> = vec![
        Box::new(drawable::Circle { radius: 5.0 }),
        Box::new(drawable::Square { side: 4.0 }),
    ];
    for shape in shapes {
        shape.draw();
    }

    println!("\n=== 3. Composition Over Inheritance ===");
    let my_car = Car::new("Toyota Crown".to_string(), 283);
    println!("{}", my_car.specs());

    println!("\n=== 4. State Pattern with Enums ===");
    let mut machine = vendingmachine::VendingMachine::new();
    machine.insert_money();
    machine.dispense_item();

    println!("\n=== 5. Builder Pattern ===");
    let pizza = builder_pattern::PizzaBuilder::new(12)
        .add_cheese()
        .add_pepperoni()
        .build();
    println!("Built pizza: size {}", pizza.size);

    println!("\n=== 6. Strategy Pattern ===");
    let cart = strategy_pattern::ShoppingCart::new(Box::new(strategy_pattern::CreditCard {
        number: "1234 5678 9012 3456".to_string(),
    }));
    cart.checkout(100.0);

    // println!("\n=== 7. Observer Pattern ===");
    // let mut station = observer_pattern::WeatherStation::new();
    // station.add_observer(Box::new(observer_pattern::Display));
    // station.set_temperature(23.5);

    // println!("\n=== 8. Decorator Pattern ===");
    // let coffee: Box<dyn decorator_pattern::Coffee> = Box::new(decorator_pattern::SimpleCoffee);
    // println!("{}: ${}", coffee.description(), coffee.cost());
    // let coffee_with_milk = decorator_pattern::MilkDecorator::new(coffee);
    // println!(
    //     "{}: ${}",
    //     coffee_with_milk.description(),
    //     coffee_with_milk.cost()
    // );

    // println!("\n=== 9. Factory Pattern ===");
    // let dog = animal_factory::AnimalFactory::create_animal(animal_factory::AnimalType::Dog);
    // dog.speak();
    // let cat = animal_factory::AnimalFactory::create_animal(animal_factory::AnimalType::Cat);
    // cat.speak();
}
