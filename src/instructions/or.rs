use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;
use std::ops::BitOr;

/// Represents the OR instruction (sets OR.0 = OR.0 | OR.1)
pub struct OR<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: BitOr<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> OR<'a, S, T, U>
where
    S: BitOr<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create OR without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        OR(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for OR<'a, S, T, U>
where
    S: BitOr<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        *self.0.write(state) = self.0.read(state) | self.1.read(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_or() {
        let mut state = State::new(&[]);
        state.registers.v_registers[1] = 0b1011_0001;
        state.registers.v_registers[13] = 0b1000_0010;
        let or = OR::new(V(B4(1)), V(B4(13)));
        or.execute(&mut state);
        assert_eq!(state.registers.v_registers[13], 0b1000_0010);
        assert_eq!(state.registers.v_registers[1], 0b1011_0011);
    }
}
