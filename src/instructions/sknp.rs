use super::{Instruction, State};
use crate::variables::Read;

/// Represents the SKNP instruction (skips next instruction if the key corresponding to SKNP.0 is
/// not pressed)
#[derive(Debug)]
pub struct SKNP<T>(pub T)
where
    T: Read<u8>;

impl<T> Instruction for SKNP<T>
where
    T: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        if !state.keyboard.is_key_pressed(self.0.read(state)) {
            state.program_counter += 2;
        }
    }
}
