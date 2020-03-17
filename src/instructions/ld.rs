use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;

/// Represents the LD instruction (loads the value of LD.1 into LD.0)
pub struct LD<S, T, U>(T, U, PhantomData<S>)
where
    T: Write<S>,
    U: Read<S>;

impl<S, T, U> LD<S, T, U>
where
    T: Write<S>,
    U: Read<S>,
{
    pub fn new(left: T, right: U) -> Self {
        LD(left, right, PhantomData)
    }
}

impl<S, T, U> Instruction for LD<S, T, U>
where
    T: Write<S>,
    U: Read<S>,
{
    fn execute(&self, state: &mut State) {
        self.0.write(state, self.1.read(state));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{byte::B8, nibble::B4, vregister::V};

    #[test]
    fn test_ld() {
        let mut state = State::mock(&[]);
        let ld = LD::new(V(B4(11)), B8::from(92));
        ld.execute(&mut state);
        assert_eq!(state.registers.v_registers[11], 92);
    }
}
