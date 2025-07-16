pub struct NotificationSender {
    recipient: String,
}

impl NotificationSender {
    pub fn new(recipient: &str) -> Self {
        Self {
            recipient: recipient.to_string(),
        }
    }

    pub fn send(&self, message: &str) {
        println!("Sending notification to {}: {}", self.recipient, message);
    }
}
