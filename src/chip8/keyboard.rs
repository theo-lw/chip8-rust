use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    EventPump,
};
use std::{collections::HashMap, thread, time::Duration};

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub trait Keyboard {
    fn is_key_pressed(&self, key: u8) -> bool;
    fn wait_for_key_press(&mut self) -> u8;
}

pub struct SDLKeyboard {
    pub u8_to_scancode: [Scancode; 16],
    pub keycode_to_u8: HashMap<Keycode, u8>,
    pub event_source: EventPump,
}

impl Keyboard for SDLKeyboard {
    fn is_key_pressed(&self, key: u8) -> bool {
        self.event_source
            .keyboard_state()
            .is_scancode_pressed(self.u8_to_scancode[usize::from(key)])
    }

    fn wait_for_key_press(&mut self) -> u8 {
        loop {
            for event in self.event_source.poll_iter() {
                if let Event::KeyDown {
                    keycode: Some(x), ..
                } = event
                {
                    return self.keycode_to_u8[&x];
                }
            }
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
