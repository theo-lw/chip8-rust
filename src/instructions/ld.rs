use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;

/// Represents the LD instruction (loads the value of LD.1 into LD.0)
pub struct LD<'a, S, T: Write<'a, S>, U: Read<S>>(T, U, PhantomData<&'a S>);

impl<'a, S, T: Write<'a, S>, U: Read<S>> LD<'a, S, T, U> {
    pub fn new(left: T, right: U) -> Self {
        LD(left, right, PhantomData)
    }
}

impl<'a, S, T: Write<'a, S>, U: Read<S>> Instruction<'a> for LD<'a, S, T, U> {
    fn execute(&self, state: &'a mut State) {
        *self.0.write(state) = self.1.read(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{byte::B8, nibble::B4, vregister::V};

    #[test]
    fn test_ld() {
        let mut state = State::new(&[]);
        let ld = LD::new(V(B4(11)), B8::from(92));
        ld.execute(&mut state);
        assert_eq!(state.registers.v_registers[11], 92);
    }
}
