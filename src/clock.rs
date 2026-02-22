use std::sync::{Arc, RwLock};

use crate::config::DisplayConfig;
use chrono::Local;

/// burp brup tiem delsply.
pub struct DisplayClock {
    config: Arc<RwLock<DisplayConfig>>,
    now: chrono::DateTime<Local>,
}

impl DisplayClock {
    pub fn new(config: Arc<RwLock<DisplayConfig>>) -> Self {
        let now = Local::now();
        Self { config, now }
    }

    /// update time
    pub fn update(&mut self) {
        self.now = Local::now();
    }

    /// get time string
    pub fn time(&self) -> String {
        match self.config.read() {
            Ok(config) => {
                let mut format = if config.show_seconds {
                    "%H:%M:%S"
                } else {
                    "%H:%M"
                }
                .to_string();

                if config.standard_time {
                    format = format.replace("%H", "%I");
                }

                self.now.format(&format).to_string()
            }
            Err(_) => String::new(),
        }
    }

    /// get date string
    pub fn date(&self) -> String {
        match self.config.read() {
            Ok(config) => {
                let format = if config.standard_time {
                    "%F [%p]"
                } else {
                    "%F"
                }
                .to_string();

                self.now.format(&format).to_string()
            }
            Err(_) => String::new(),
        }
    }
}
