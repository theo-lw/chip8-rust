use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;
use std::ops::BitAnd;

/// Represents the AND instruction (sets AND.0 = AND.0 | AND.1)
pub struct AND<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: BitAnd<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> AND<'a, S, T, U>
where
    S: BitAnd<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create AND without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        AND(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for AND<'a, S, T, U>
where
    S: BitAnd<Output = S>,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        *self.0.write(state) = self.0.read(state) & self.1.read(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_and() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[3] = 0b1001_1001;
        state.registers.v_registers[5] = 0b1010_1011;
        let and = AND::new(V(B4(3)), V(B4(5)));
        and.execute(&mut state);
        assert_eq!(state.registers.v_registers[3], 0b1000_1001);
        assert_eq!(state.registers.v_registers[5], 0b1010_1011);
    }
}
