use super::{Read, State};

pub struct K;

impl Read<u8> for K {
    fn read(&self, state: &State) -> u8 {
        state.keyboard.wait_for_key_press()
    }
}
