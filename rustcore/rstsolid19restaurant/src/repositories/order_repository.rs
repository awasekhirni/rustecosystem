use crate::models::Order;
use crate::traits::Repository;
use std::error::Error;

/// Repository for order persistence
///
/// Demonstrates Interface Segregation Principle (ISP) -
/// implements only the Repository trait methods it needs
pub struct OrderRepository {
    // In a real app, this would hold a database connection
    orders: Vec<Order>,
}

impl OrderRepository {
    pub fn new() -> Self {
        OrderRepository { orders: Vec::new() }
    }
}

impl Repository<Order> for OrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), Box<dyn Error>> {
        println!("Saving order #{} to repository", order.id());
        // Clone the order since we have a reference
        self.orders.push(order.clone());
        Ok(())
    }

    fn find_by_id(&self, id: u64) -> Result<Option<Order>, Box<dyn Error>> {
        Ok(self.orders.iter().find(|o| o.id() == id).cloned())
    }
}
