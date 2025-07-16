// Notifier trait (Interface Segregation Principle - I in SOLID)
pub trait Notifier: Send + Sync {
    fn notify(&self, message: &str);
}

// Email notifier implementation
pub struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) {
        println!("Sending email notification: {}", message);
    }
}

// SMS notifier implementation
pub struct SmsNotifier;

impl Notifier for SmsNotifier {
    fn notify(&self, message: &str) {
        println!("Sending SMS notification: {}", message);
    }
}

// Composite notifier that can contain multiple notifiers (Liskov Substitution - L in SOLID)
pub struct CompositeNotifier {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl CompositeNotifier {
    pub fn new(notifiers: Vec<Box<dyn Notifier>>) -> Self {
        CompositeNotifier { notifiers }
    }
}

impl Notifier for CompositeNotifier {
    fn notify(&self, message: &str) {
        for notifier in &self.notifiers {
            notifier.notify(message);
        }
    }
}
