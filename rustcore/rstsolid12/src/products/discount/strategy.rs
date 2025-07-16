use crate::products::product::Product;

/// DiscountStrategy trait defines the interface for all discount strategies
/// This follows the Open/Closed Principle (OCP) as new strategies can be added without modifying existing code
pub trait DiscountStrategy {
    fn apply_discount(&self, product: &Product) -> f64;
}

/// NoDiscount strategy applies no discount
pub struct NoDiscount;

impl DiscountStrategy for NoDiscount {
    fn apply_discount(&self, product: &Product) -> f64 {
        product.price
    }
}

/// PercentageDiscount strategy applies a percentage discount
pub struct PercentageDiscount {
    percentage: f64,
}

impl PercentageDiscount {
    pub fn new(percentage: f64) -> Self {
        PercentageDiscount { percentage }
    }
}

impl DiscountStrategy for PercentageDiscount {
    fn apply_discount(&self, product: &Product) -> f64 {
        product.price * (1.0 - self.percentage / 100.0)
    }
}

/// CategoryDiscount strategy applies discount based on product category
pub struct CategoryDiscount {
    category: String,
    percentage: f64,
}

impl CategoryDiscount {
    pub fn new(category: String, percentage: f64) -> Self {
        CategoryDiscount {
            category,
            percentage,
        }
    }
}

impl DiscountStrategy for CategoryDiscount {
    fn apply_discount(&self, product: &Product) -> f64 {
        if product.category == self.category {
            product.price * (1.0 - self.percentage / 100.0)
        } else {
            product.price
        }
    }
}
