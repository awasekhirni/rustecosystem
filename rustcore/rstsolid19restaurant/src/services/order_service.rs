use crate::models::Order;
use crate::traits::{OrderProcessor, Repository};
use std::error::Error;

/// Service for processing orders
///
/// Demonstrates Dependency Inversion Principle (DIP) -
/// depends on abstractions (traits) rather than concrete implementations
pub struct OrderService<R, P>
where
    R: Repository<Order>,
    P: OrderProcessor,
{
    order_repository: Box<R>,
    payment_service: Box<P>,
}

impl<R, P> OrderService<R, P>
where
    R: Repository<Order>,
    P: OrderProcessor,
{
    /// Creates a new OrderService with injected dependencies
    pub fn new(order_repository: Box<R>, payment_service: Box<P>) -> Self {
        OrderService {
            order_repository,
            payment_service,
        }
    }

    /// Processes an order through all steps
    ///
    /// Demonstrates Open/Closed Principle (OCP) -
    /// we can extend behavior by implementing new processors/repositories
    /// without modifying this code
    // src/services/order_service.rs
    pub fn process_order(&mut self, mut order: Order) -> Result<Order, Box<dyn Error>> {
        // Changed &self to &mut self to allow mutation
        self.order_repository.save(&order)?;
        let total = order.total_amount();
        self.payment_service.process_payment(total)?;
        order.set_status(crate::models::OrderStatus::Completed);
        self.order_repository.save(&order)?;
        Ok(order)
    }
}
