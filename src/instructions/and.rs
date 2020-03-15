use super::{Instruction, State};
use crate::variables::{Read, Write};

/// Represents the AND instruction (sets AND.0 = AND.0 | AND.1)
pub struct AND<T, U>(pub T, pub U)
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>;

impl<T, U> Instruction for AND<T, U>
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        self.0.write(state, self.0.read(state) & self.1.read(state));
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
        let and = AND(V(B4(3)), V(B4(5)));
        and.execute(&mut state);
        assert_eq!(state.registers.v_registers[3], 0b1000_1001);
        assert_eq!(state.registers.v_registers[5], 0b1010_1011);
    }
}
