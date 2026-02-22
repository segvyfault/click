use std::sync::{Arc, RwLock};
use std::time::Duration;

use ratatui::crossterm::event::{self, KeyCode};

use crate::config::DisplayConfig;

pub struct EventHandler {
    display_config: Arc<RwLock<DisplayConfig>>,
    should_quit: Arc<RwLock<bool>>,
}

impl EventHandler {
    pub fn new(display_config: Arc<RwLock<DisplayConfig>>) -> (Self, Arc<RwLock<bool>>) {
        let should_quit = Arc::new(RwLock::new(false));

        (
            Self {
                should_quit: should_quit.clone(),
                display_config,
            },
            should_quit,
        )
    }

    pub fn handle_events(&self) -> Result<(), Box<dyn std::error::Error>> {
        let event = event::poll(Duration::from_millis(100))?;

        if event {
            match event::read()? {
                event::Event::Key(key) if key.is_press() => match key.code {
                    KeyCode::Char('q') => {
                        if let Ok(mut quit) = self.should_quit.write() {
                            *quit = true;
                        }
                    }

                    KeyCode::Char('c') => {
                        if let Ok(mut config) = self.display_config.write() {
                            config.centerize_text = !config.centerize_text;
                        }
                    }

                    KeyCode::Char('s') => {
                        if let Ok(mut config) = self.display_config.write() {
                            config.show_seconds = !config.show_seconds;
                        }
                    }

                    KeyCode::Char('d') => {
                        if let Ok(mut config) = self.display_config.write() {
                            config.hide_date = !config.hide_date;
                        }
                    }

                    KeyCode::Char('t') => {
                        if let Ok(mut config) = self.display_config.write() {
                            config.standard_time = !config.standard_time;
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        Ok(())
    }
}
