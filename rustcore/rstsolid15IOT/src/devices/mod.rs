//! Devices module exposing all device-related functionality
//!
//! Demonstrates:
//! - DIP: Higher-level modules depend on Device trait, not concrete implementations

pub use self::actuator::Actuator;
pub use self::sensor::Sensor;
pub use self::traits::{Calibratable, Device, Diagnosable};

mod actuator;
mod sensor;
mod traits;
