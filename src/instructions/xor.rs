use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;
use std::ops::BitXor;

/// Represents the XOR instruction (sets XOR.0 = XOR.0 ^ XOR.1)
pub struct XOR<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: BitXor<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> XOR<'a, S, T, U>
where
    S: BitXor<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create XOR without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        XOR(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for XOR<'a, S, T, U>
where
    S: BitXor<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        *self.0.write(state) = self.0.read(state) ^ self.1.read(state);
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
        let xor = XOR::new(V(B4(0x0)), V(B4(0xE)));
        xor.execute(&mut state);
        assert_eq!(state.registers.v_registers[0x0], 0b0011_1010);
        assert_eq!(state.registers.v_registers[0xE], 0b1010_0011);
    }
}
