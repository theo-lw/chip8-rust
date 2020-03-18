use super::{Instruction, State};
use crate::variables::{Read, Write};
use rand::random;

/// Represents the RND instruction (set RND.0 to a random byte & RND.1)
#[derive(Debug)]
pub struct RND<T, U>(pub T, pub U)
where
    T: Write<u8>,
    U: Read<u8>;

impl<T, U> Instruction for RND<T, U>
where
    T: Write<u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        self.0.write(state, random::<u8>() & self.1.read(state));
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
        let rnd = RND(V(B4(3)), B8::from(0));
        rnd.execute(&mut state);
        assert_eq!(state.registers.v_registers[3], 0);
    }
}
