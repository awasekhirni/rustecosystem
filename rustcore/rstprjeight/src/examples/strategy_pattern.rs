//Strategy pattern

pub trait PaymentStrategy {
    fn pay(&self, amount: f64) -> bool;
}

pub struct CreditCard {
    pub number: String,
}

impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: f64) -> bool {
        println!("Processing credit card {} for ${}", self.number, amount);
        true
    }
}

pub struct PayPal {
    email: String,
}

impl PaymentStrategy for PayPal {
    fn pay(&self, amount: f64) -> bool {
        println!(
            "Processing PayPal payment from {} for ${}",
            self.email, amount
        );
        true
    }
}

pub struct ShoppingCart {
    payment_method: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    pub fn new(payment_method: Box<dyn PaymentStrategy>) -> Self {
        ShoppingCart { payment_method }
    }

    pub fn checkout(&self, amount: f64) {
        self.payment_method.pay(amount);
    }
}

// let cart = ShoppingCart::new(Box::new(CreditCard {
//     number: "1234 5678 9012 3456".to_string(),
// }));
// cart.checkout(100.0);
