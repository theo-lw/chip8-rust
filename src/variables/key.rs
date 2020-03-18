use super::{Read, State};

/// Represents the next key press
#[derive(Debug)]
pub struct K;

impl Read<u8> for K {
    fn read(&self, state: &State) -> u8 {
        state.keyboard.wait_for_key_press()
    }
}
