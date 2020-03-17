use super::{Instruction, State};

/// Represents the CLS instruction (clears the display)
pub struct CLS;

impl Instruction for CLS {
    fn execute(&self, state: &mut State) {
        state.display.clear();
    }
}
