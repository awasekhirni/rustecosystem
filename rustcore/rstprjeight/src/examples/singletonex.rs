//singpleton pattern with lazy static
use lazy_static::lazy_static;
use std::sync::Mutex;

struct AppConfig {
    settings: String,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig {
            settings: "default settings".to_string(),
        }
    }

    fn get_settings(&self) -> &str {
        &self.settings
    }

    fn update_settings(&mut self, new_settings: String) {
        self.settings = new_settings;
    }
}

lazy_static! {
    static ref CONFIG: Mutex<AppConfig> = Mutex::new(AppConfig::new());
}

// fn main() {
//     {
//         let mut config = CONFIG.lock().unwrap();
//         config.update_settings("new settings".to_string());
//     }

//     let config = CONFIG.lock().unwrap();
//     println!("Current settings: {}", config.get_settings());
// }
