use super::traits::{Calibratable, Device, Diagnosable};
use rand::Rng;

pub struct Sensor {
    id: String,
    sensor_type: String,
    calibration_offset: f64,
    min_value: f64,
    max_value: f64,
}

impl Sensor {
    pub fn new(id: &str, sensor_type: &str) -> Self {
        Self {
            id: id.to_string(),
            sensor_type: sensor_type.to_string(),
            calibration_offset: 0.0,
            min_value: 0.0,
            max_value: 100.0,
        }
    }

    pub fn with_value_range(mut self, min: f64, max: f64) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }
}

impl Device for Sensor {
    fn read(&mut self) -> f64 {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(self.min_value..self.max_value);
        value + self.calibration_offset
    }

    fn write(&mut self, _value: f64) {
        // Sensors typically don't accept writes
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_type(&self) -> &str {
        &self.sensor_type
    }
}

impl Calibratable for Sensor {
    fn calibrate(&mut self, offset: f64) {
        self.calibration_offset = offset;
    }
}

impl Diagnosable for Sensor {
    fn run_diagnostics(&self) -> String {
        format!(
            "Sensor {} ({}): OK. Calibration offset: {:.2}",
            self.id, self.sensor_type, self.calibration_offset
        )
    }
}
