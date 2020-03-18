use super::{Instruction, State};
use crate::variables::{Read, Write};

/// Represents the SHL instruction (bitwise left-shift on the contents of SHL.0)
#[derive(Debug)]
pub struct SHL<T>(pub T)
where
    T: Read<u8> + Write<u8>;

impl<T> Instruction for SHL<T>
where
    T: Read<u8> + Write<u8>,
{
    fn execute(&self, state: &mut State) {
        let val = self.0.read(state);
        state.registers.v_registers[0xF] = (val & 0b1000_0000) >> 7;
        self.0.write(state, val << 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_shl_no_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[5] = 0b0101_0111;
        let shl = SHL(V(B4(5)));
        shl.execute(&mut state);
        assert_eq!(state.registers.v_registers[5], 0b1010_1110);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }

    #[test]
    fn test_shl_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[8] = 0b1100_0101;
        let shl = SHL(V(B4(8)));
        shl.execute(&mut state);
        assert_eq!(state.registers.v_registers[8], 0b1000_1010);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }
}
