// src/models/order.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    id: u64,
    items: Vec<super::MenuItem>,
    status: OrderStatus,
}

impl Order {
    pub fn new(id: u64) -> Self {
        Order {
            id,
            items: Vec::new(),
            status: OrderStatus::Pending,
        }
    }

    pub fn add_item(&mut self, item: super::MenuItem) {
        self.items.push(item);
    }

    pub fn total_amount(&self) -> f64 {
        self.items.iter().map(|item| item.price()).sum()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: OrderStatus) {
        self.status = status;
    }
}
