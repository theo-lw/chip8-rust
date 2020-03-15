use super::{Instruction, State};
use crate::variables::{Read, Write};

/// Represents the XOR instruction (sets XOR.0 = XOR.0 ^ XOR.1)
pub struct XOR<T, U>(pub T, pub U)
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>;

impl<T, U> Instruction for XOR<T, U>
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        self.0.write(state, self.0.read(state) ^ self.1.read(state));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_xor() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[0x0] = 0b1001_1001;
        state.registers.v_registers[0xE] = 0b1010_0011;
        let xor = XOR(V(B4(0x0)), V(B4(0xE)));
        xor.execute(&mut state);
        assert_eq!(state.registers.v_registers[0x0], 0b0011_1010);
        assert_eq!(state.registers.v_registers[0xE], 0b1010_0011);
    }
}
