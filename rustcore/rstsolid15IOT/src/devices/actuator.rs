//! Actuator device implementation
//!
//! Demonstrates:
//! - SRP: Only handles actuator-related functionality
//! - ISP: Implements only needed traits (not Calibratable in this case)

use super::traits::{Device, Diagnosable};

/// An actuator device that can be turned on/off
pub struct Actuator {
    id: String,
    actuator_type: String,
    current_state: f64,
}

impl Actuator {
    /// Creates a new actuator
    pub fn new(id: &str, actuator_type: &str) -> Self {
        Self {
            id: id.to_string(),
            actuator_type: actuator_type.to_string(),
            current_state: 0.0,
        }
    }
}

impl Device for Actuator {
    fn read(&mut self) -> f64 {
        self.current_state
    }

    fn write(&mut self, value: f64) {
        self.current_state = if value > 0.5 { 1.0 } else { 0.0 };
        println!(
            "Actuator {} ({}) set to {}",
            self.id,
            self.actuator_type,
            if self.current_state > 0.5 {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_type(&self) -> &str {
        &self.actuator_type
    }
}

impl Diagnosable for Actuator {
    fn run_diagnostics(&self) -> String {
        format!(
            "Actuator {} ({}): {}. Current state: {:.1}",
            self.id,
            self.actuator_type,
            if self.current_state > 0.5 {
                "ACTIVE"
            } else {
                "INACTIVE"
            },
            self.current_state
        )
    }
}
