use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    EventPump,
};
use std::{cell::RefCell, collections::HashMap, thread, time::Duration};

#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

/// A trait that encapsulates the functionality required for keyboard IO.
/// This trait exists to let us mock the keyboard in unit tests
/// The chip-8 keyboard has 16 hexadecimal keys.
#[cfg_attr(test, automock)]
pub trait Keyboard {
    fn is_key_pressed(&self, key: u8) -> bool;
    fn wait_for_key_press(&self) -> u8;
    fn is_quit(&self) -> bool;
}

/// A struct that implements the Keyboard trait using the SDL2 library
pub struct SDLKeyboard {
    /// Map from u8 to SDL2 Keycodes
    u8_to_keycode: [Keycode; 16],
    /// Map from SDL2 Keycodes to u8s
    keycode_to_u8: HashMap<Keycode, u8>,
    event_source: RefCell<EventPump>,
}

impl SDLKeyboard {
    pub fn new(events: EventPump, u8_to_keycode: [Keycode; 16]) -> Self {
        let keycode_to_u8: HashMap<Keycode, u8> = u8_to_keycode
            .iter()
            .enumerate()
            .map(|(key, val)| (*val, key as u8))
            .collect();
        SDLKeyboard {
            u8_to_keycode,
            keycode_to_u8,
            event_source: RefCell::new(events),
        }
    }
}

impl Keyboard for SDLKeyboard {
    fn is_key_pressed(&self, key: u8) -> bool {
        let scancode = Scancode::from_keycode(self.u8_to_keycode[usize::from(key)]).unwrap();
        self.event_source
            .borrow()
            .keyboard_state()
            .is_scancode_pressed(scancode)
    }

    fn wait_for_key_press(&self) -> u8 {
        loop {
            for event in self.event_source.borrow_mut().poll_iter() {
                if let Event::KeyDown {
                    keycode: Some(x), ..
                } = event
                {
                    if self.keycode_to_u8.contains_key(&x) {
                        return self.keycode_to_u8[&x];
                    }
                }
            }
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn is_quit(&self) -> bool {
        for event in self.event_source.borrow_mut().poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                _ => {}
            }
        }
        return false;
    }
}
