use sdl2::{keyboard::Keycode, pixels::Color};
use serde::Deserialize;
use serde_json;
use std::{collections::HashMap, fs::File, io::BufReader};

/// Represents the JSON config file
#[derive(Deserialize, Clone)]
pub struct Config {
    pub ticks_per_frame: u8,
    pub pixel_size: u32,
    active_color: ColorConfig,
    inactive_color: ColorConfig,
    keyboard: HashMap<String, String>,
}

impl Config {
    /// Reads a config struct from a file path
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file at {}", path));
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .unwrap_or_else(|_| panic!("Could not deserialize file at {}", path))
    }

    /// Returns the default key mappings for the keys not given in the config file
    fn default_keyboard() -> HashMap<String, String> {
        [
            ("1", "1"),
            ("2", "2"),
            ("3", "3"),
            ("C", "4"),
            ("4", "Q"),
            ("5", "W"),
            ("6", "E"),
            ("D", "R"),
            ("7", "A"),
            ("8", "S"),
            ("9", "D"),
            ("E", "F"),
            ("A", "Z"),
            ("0", "X"),
            ("B", "C"),
            ("F", "V"),
        ]
        .iter()
        .map(|(x, y)| (String::from(*x), String::from(*y)))
        .collect()
    }

    /// Returns an array of keycodes representing the keyboard mapping (for SDL)
    /// The index of an element is its chip8 keycode, the element itself is the SDL keycode
    pub fn get_keyboard(&self) -> [Keycode; 16] {
        let mut result = [Keycode::Num2; 16];
        let default_keyboard = Self::default_keyboard();
        for i in 0..16 {
            let key_name: &String = if self.keyboard.contains_key(&format!("{:x}", i)) {
                &self.keyboard[&format!("{:X}", i)]
            } else {
                &default_keyboard[&format!("{:X}", i)]
            };
            result[i] = Keycode::from_name(key_name).unwrap_or_else(|| {
                panic!(
                    "Could not find key with name {}. Please use an SDL key name!",
                    key_name
                )
            });
        }
        result
    }

    /// Gets the active color (for SDL)
    pub fn get_active_color(&self) -> Color {
        Color::from(self.active_color)
    }

    /// Gets the inactive color (for SDL)
    pub fn get_inactive_color(&self) -> Color {
        Color::from(self.inactive_color)
    }
}

/// Default configuration
impl Default for Config {
    fn default() -> Self {
        Config {
            ticks_per_frame: 1,
            pixel_size: 10,
            active_color: ColorConfig::white(),
            inactive_color: ColorConfig::black(),
            keyboard: Self::default_keyboard(),
        }
    }
}

/// Struct representing a color
#[derive(Deserialize, Copy, Clone, Debug)]
struct ColorConfig {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorConfig {
    /// Creates a white color config
    fn white() -> Self {
        ColorConfig {
            r: u8::max_value(),
            g: u8::max_value(),
            b: u8::max_value(),
        }
    }

    /// Creates a black color config
    fn black() -> Self {
        ColorConfig {
            r: u8::min_value(),
            g: u8::min_value(),
            b: u8::min_value(),
        }
    }
}

impl From<ColorConfig> for Color {
    fn from(config: ColorConfig) -> Self {
        Color::RGB(config.r, config.g, config.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_keyboard() {
        let config: Config = Default::default();
        assert_eq!(config.get_keyboard().len(), 16);
    }
}
