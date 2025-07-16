/// ReportGenerator trait defines the interface for generating reports
/// Demonstrates Interface Segregation Principle - small, focused interface
pub trait ReportGenerator {
    /// Generates a report as a String
    fn generate_report(&self) -> String;
}
