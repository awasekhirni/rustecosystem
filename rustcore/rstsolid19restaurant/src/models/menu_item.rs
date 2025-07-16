// src/models/menu_item.rs
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    name: String,
    price: f64,
}

impl MenuItem {
    pub fn new(name: &str, price: f64) -> Self {
        MenuItem {
            name: name.to_string(),
            price,
        }
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
