use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;

/// Represents the SHR instruction (bitwise right-shift on the contents of SHR.0)
pub struct SHR<'a, T>(T, PhantomData<&'a u8>)
where
    T: Read<u8> + Write<'a, u8>;

impl<'a, T> SHR<'a, T>
where
    T: Read<u8> + Write<'a, u8>,
{
    /// Convenience constructor so we don't have to type out PhantomData
    pub fn new(val: T) -> Self {
        SHR(val, PhantomData)
    }
}

impl<'a, T> Instruction<'a> for SHR<'a, T>
where
    T: Read<u8> + Write<'a, u8>,
{
    fn execute(&self, state: &'a mut State) {
        let val = self.0.read(state);
        state.registers.v_registers[0xF] = val & 1;
        *self.0.write(state) = val >> 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_shr_no_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[5] = 0b1001_0110;
        let shr = SHR::new(V(B4(5)));
        shr.execute(&mut state);
        assert_eq!(state.registers.v_registers[5], 0b0100_1011);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }

    #[test]
    fn test_shr_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[8] = 0b1100_0101;
        let shr = SHR::new(V(B4(8)));
        shr.execute(&mut state);
        assert_eq!(state.registers.v_registers[8], 0b0110_0010);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }
}
