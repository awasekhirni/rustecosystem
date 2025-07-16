//! Device traits defining interfaces for IoT devices
//!
//! Demonstrates:
//! - ISP (Interface Segregation): Separate traits for different capabilities
//! - LSP (Liskov Substitution): Concrete devices can substitute trait objects

/// Common functionality for all IoT devices
pub trait Device {
    /// Reads the current value from the device
    fn read(&mut self) -> f64;

    /// Writes a value to the device (if applicable)
    fn write(&mut self, value: f64);

    /// Gets the device ID
    fn get_id(&self) -> &str;

    /// Gets the device type
    fn get_type(&self) -> &str;
}

/// Trait for devices that can be calibrated
pub trait Calibratable {
    /// Calibrates the device with a given offset
    fn calibrate(&mut self, offset: f64);
}

/// Trait for devices that provide diagnostic information
pub trait Diagnosable {
    /// Runs self-diagnostics and returns status
    fn run_diagnostics(&self) -> String;
}
