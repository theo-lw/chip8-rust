use super::{Instruction, State};
use crate::variables::Read;

/// Represents the SKP instruction (skips next instruction if the key corresponding to SKP.0 is
/// pressed)
#[derive(Debug)]
pub struct SKP<T>(pub T)
where
    T: Read<u8>;

impl<T> Instruction for SKP<T>
where
    T: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        if state.keyboard.is_key_pressed(self.0.read(state)) {
            state.program_counter += 2;
        }
    }
}
