//observer design pattern
trait Observer {
    fn update(&self, temperature: f64);
}

trait Observable {
    fn add_observer(&mut self, observer: Box<dyn Observer>);
    fn notify_observers(&self);
}

pub struct WeatherStation {
    observers: Vec<Box<dyn Observer>>,
    temperature: f64,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation {
            observers: Vec::new(),
            temperature: 0.0,
        }
    }

    fn set_temperature(&mut self, temp: f64) {
        self.temperature = temp;
        self.notify_observers();
    }

    // pub fn add_observer(&self, display: Box<_>) -> _ {
    //     todo!()
    // }
}

impl Observable for WeatherStation {
    fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.update(self.temperature);
        }
    }
}

pub struct Display;

impl Observer for Display {
    fn update(&self, temperature: f64) {
        println!("Temperature updated: {}°C", temperature);
    }
}

// let mut station = WeatherStation::new();
// station.add_observer(Box::new(Display));
// station.set_temperature(23.5);
