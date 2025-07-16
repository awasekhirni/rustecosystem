/// Notification trait
pub trait Notifiable {
    fn send_notification(&self, message: &str) -> bool;
}
