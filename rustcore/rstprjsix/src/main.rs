use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    stock: u32,
    discount: f64, // Discount as a percentage (e.g., 20.0 for 20%)
}

impl Product {
    fn apply_discount(&mut self) {
        let discount_amount = self.price * (self.discount / 100.0);
        self.price -= discount_amount;
    }

    fn calculate_tax(&self, tax_rate: f64) -> f64 {
        self.price * (tax_rate / 100.0)
    }

    fn total_price_with_tax(&self, tax_rate: f64) -> f64 {
        self.price + self.calculate_tax(tax_rate)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Inventory {
    products: HashMap<u32, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    fn update_stock(&mut self, id: u32, quantity: u32) {
        if let Some(product) = self.products.get_mut(&id) {
            product.stock = quantity;
        }
    }

    fn apply_discount_to_all(&mut self) {
        for product in self.products.values_mut() {
            product.apply_discount();
        }
    }

    fn calculate_total_inventory_value(&self, tax_rate: f64) -> f64 {
        self.products
            .values()
            .map(|product| {
                let total_price = product.total_price_with_tax(tax_rate);
                total_price * product.stock as f64
            })
            .sum()
    }
}

fn main() {
    // Create a new inventory
    let mut inventory = Inventory::new();

    // Add products to the inventory
    inventory.add_product(Product {
        id: 1,
        name: "Laptop".to_string(),
        price: 1000.0,
        stock: 10,
        discount: 10.0,
    });
    inventory.add_product(Product {
        id: 2,
        name: "Smartphone".to_string(),
        price: 500.0,
        stock: 20,
        discount: 5.0,
    });

    // Apply discounts to all products
    inventory.apply_discount_to_all();

    // Calculate total inventory value with tax
    let tax_rate = 15.0; // 15%
    let total_value = inventory.calculate_total_inventory_value(tax_rate);
    println!("Total inventory value with tax: ${:.2}", total_value);

    // Serialize inventory to JSON
    let serialized = serde_json::to_string(&inventory).unwrap();
    println!("Serialized inventory: {}", serialized);

    // Deserialize inventory from JSON
    let deserialized: Inventory = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized inventory: {:?}", deserialized);
}
