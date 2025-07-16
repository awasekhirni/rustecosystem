use std::fmt;

#[derive(Debug, Clone)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl Customer {
    pub fn new(id: u32, name: &str, email: &str, phone: &str) -> Self {
        Customer {
            id,
            name: name.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
        }
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Customer {}: {} ({} | {})",
            self.id, self.name, self.email, self.phone
        )
    }
}
