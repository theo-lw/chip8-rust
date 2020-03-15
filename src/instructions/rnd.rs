use super::{Instruction, State};
use crate::variables::{Read, Write};
use rand::random;
use std::marker::PhantomData;

/// Represents the RND instruction (set RND.0 to a random byte & RND.1)
pub struct RND<'a, T, U>(T, U, PhantomData<&'a u8>)
where
    T: Write<'a, u8>,
    U: Read<u8>;

impl<'a, T, U> RND<'a, T, U>
where
    T: Write<'a, u8>,
    U: Read<u8>,
{
    /// Convenience constructor so we don't have to type out PhantomData
    pub fn new(left: T, right: U) -> Self {
        RND(left, right, PhantomData)
    }
}

impl<'a, T, U> Instruction<'a> for RND<'a, T, U>
where
    T: Write<'a, u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &'a mut State) {
        *self.0.write(state) = random::<u8>() & self.1.read(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{byte::B8, nibble::B4, vregister::V};

    #[test]
    fn test_rnd_basic() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[3] = 4;
        let rnd = RND::new(V(B4(3)), B8::from(0));
        rnd.execute(&mut state);
        assert_eq!(state.registers.v_registers[3], 0);
    }
}
