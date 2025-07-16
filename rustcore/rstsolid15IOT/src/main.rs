//! Simple IoT Application demonstrating SOLID principles in Rust

mod data;
mod devices;
mod notifications;

use crate::data::DataProcessor;
use crate::devices::{Actuator, Device, Sensor};
use crate::notifications::NotificationSender;
use std::time::Duration;

fn main() {
    let temperature_sensor = Sensor::new("temp_sensor_1", "temperature");
    let led_actuator = Actuator::new("led_1", "light");

    let data_processor = DataProcessor::new(|reading| reading * 1.8 + 32.0);

    let notification_sender = NotificationSender::new("iot-alerts@example.com");

    simulate_iot_operation(
        temperature_sensor,
        led_actuator,
        data_processor,
        notification_sender,
    );
}

fn simulate_iot_operation(
    mut sensor: impl Device,
    mut actuator: impl Device,
    data_processor: DataProcessor,
    notification_sender: NotificationSender,
) {
    println!("Starting IoT system simulation...");

    for i in 1..=5 {
        println!("\nCycle {}:", i);

        let reading = sensor.read();
        println!("Sensor reading: {:.2}", reading);

        let processed = data_processor.process(reading);
        println!("Processed data: {:.2}", processed);

        if processed > 90.0 {
            notification_sender.send(&format!("High temperature alert: {:.2}F", processed));
            actuator.write(1.0);
        } else {
            actuator.write(0.0);
        }

        std::thread::sleep(Duration::from_secs(1));
    }

    println!("\nSimulation complete.");
}
