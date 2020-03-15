use super::{Instruction, State};
use crate::variables::{Read, Write};

/// Represents the OR instruction (sets OR.0 = OR.0 | OR.1)
pub struct OR<T, U>(pub T, pub U)
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>;

impl<T, U> Instruction for OR<T, U>
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        self.0.write(state, self.0.read(state) | self.1.read(state));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_or() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[1] = 0b1011_0001;
        state.registers.v_registers[13] = 0b1000_0010;
        let or = OR(V(B4(1)), V(B4(13)));
        or.execute(&mut state);
        assert_eq!(state.registers.v_registers[13], 0b1000_0010);
        assert_eq!(state.registers.v_registers[1], 0b1011_0011);
    }
}
