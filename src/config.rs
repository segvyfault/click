use std::fs;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use clap::parser::ValueSource;
use clap::{Arg, ArgAction, Command, value_parser};
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct ColorConfig {
    /// Text color
    #[serde(default = "Color::default")]
    foreground_color: Color,
    // /// If not set then transparent
    // sorta like nvims background
    // background_color: Option<Color>,
}

#[derive(Deserialize, Default, Debug)]
pub struct DisplayConfig {
    /// Shows seconds alongside minutes
    #[serde(default)]
    pub show_seconds: bool,
    /// Makes sure text is at the center of the terminal
    #[serde(default)]
    pub centerize_text: bool,
    /// true for 12 hour format
    #[serde(default)]
    pub standard_time: bool,
    /// Hides todays date below the clock
    #[serde(default)]
    pub hide_date: bool,
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigBuilder {
    colors: ColorConfig,
    display: DisplayConfig,
}

#[test]
fn toml_decode() {
    if let Ok(config) = fs::read_to_string(format!(
        "{}/.config/click/config.toml",
        ConfigBuilder::home()
    )) {
        println!("contents = {config:?}");
        let config: ConfigBuilder = toml::from_str(&config).expect("Invalid config");
        println!("config = {config:?}");
    }
}

impl ConfigBuilder {
    fn home() -> String {
        std::env::var("HOME").expect("you homeless or sum")
    }

    pub fn new() -> Self {
        if let Ok(config) =
            fs::read_to_string(format!("{}/.config/click/config.toml", Self::home()))
        {
            toml::from_str(&config).expect("Invalid config")
        } else {
            Self::default()
        }
    }

    pub fn build(self) -> Config {
        Config::from(self)
    }

    pub fn parse_cli(mut self) -> Self {
        let matches = Command::new("click")
            .about("Cli-Clock, the terminal clock :3")
            .arg(
                Arg::new("fg")
                    .long("fg")
                    .help("Text color, must be in \"#FFFFFF\" format")
                    .required(false)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(String)),
            )
            .arg(
                Arg::new("show-seconds")
                    .short('s')
                    .help("Shows seconds alongside minutes and hours, false by default")
                    .default_missing_value("true")
                    .action(ArgAction::SetTrue)
                    .value_parser(value_parser!(bool)),
            )
            .arg(
                Arg::new("centerize-text")
                    .short('c')
                    .help("Makes the time centerized, false by default")
                    .default_missing_value("true")
                    .action(ArgAction::SetTrue)
                    .value_parser(value_parser!(bool)),
            )
            .arg(
                Arg::new("standard-time")
                    .short('t')
                    .help("12 hour format for the time, false by default")
                    .default_missing_value("true")
                    .action(ArgAction::SetTrue)
                    .value_parser(value_parser!(bool)),
            )
            .arg(
                Arg::new("hide-date")
                    .short('d')
                    .help("Hides the date, false by default")
                    .default_missing_value("true")
                    .action(ArgAction::SetTrue)
                    .value_parser(value_parser!(bool)),
            )
            .get_matches();

        if let Some(color) = matches.get_one::<String>("fg") {
            let mut color = color.replace("0x", "#");
            if !color.starts_with("#") {
                color.insert(0, '#');
            }

            self.colors.foreground_color = Color::from_str(&color).expect("Expected a valid color")
        }

        if let Some(source) = matches.value_source("show-seconds")
            && source != ValueSource::DefaultValue
        {
            self.display.show_seconds = matches.get_flag("show-seconds");
        }

        if let Some(source) = matches.value_source("centerize-text")
            && source != ValueSource::DefaultValue
        {
            self.display.centerize_text = matches.get_flag("centerize-text");
        }

        if let Some(source) = matches.value_source("standard-time")
            && source != ValueSource::DefaultValue
        {
            self.display.standard_time = matches.get_flag("standard-time");
        }

        if let Some(source) = matches.value_source("hide-date")
            && source != ValueSource::DefaultValue
        {
            self.display.hide_date = matches.get_flag("hide-date");
        }

        self
    }
}

#[derive(Debug)]
pub struct Config {
    colors: ColorConfig,
    display: Arc<RwLock<DisplayConfig>>,
}

impl From<ConfigBuilder> for Config {
    fn from(value: ConfigBuilder) -> Self {
        Self {
            colors: value.colors,
            display: Arc::new(RwLock::new(value.display)),
        }
    }
}

impl Config {
    /// builder automatically tries to parse the config
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn fg(&self) -> &Color {
        &self.colors.foreground_color
    }

    pub fn display_config(&self) -> Arc<RwLock<DisplayConfig>> {
        self.display.clone()
    }
}
