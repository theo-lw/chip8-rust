use super::{Instruction, State};
use crate::variables::Read;

/// Represents the SNE instruction (skip the next instruction if SNE.0 != SNE.1)
#[derive(Debug)]
pub struct SNE<T: Read<u8>, U: Read<u8>>(pub T, pub U);

impl<T: Read<u8>, U: Read<u8>> Instruction for SNE<T, U> {
    fn execute(&self, state: &mut State) {
        if self.0.read(state) != self.1.read(state) {
            state.program_counter += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{byte::B8, nibble::B4, vregister::V};

    #[test]
    fn test_sne_true() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[4] = 98;
        let se = SNE(V(B4(4)), B8::from(98));
        let program_counter = state.program_counter;
        se.execute(&mut state);
        assert_eq!(state.program_counter, program_counter);
    }

    #[test]
    fn test_sne_false() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[11] = 9;
        state.registers.v_registers[2] = 44;
        let se = SNE(V(B4(11)), V(B4(2)));
        let program_counter = state.program_counter;
        se.execute(&mut state);
        assert_eq!(state.program_counter, program_counter + 2);
    }
}
