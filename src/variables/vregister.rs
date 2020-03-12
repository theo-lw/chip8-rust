use super::State;
use super::{Read, Write};

/// Struct representing the contents at a v-register
/// There are only 16 v-registers, numbered from 0..16
/// It is expected that this struct is constructed with a value
/// that returns a number in the range 0..16 when read.
#[derive(Debug, Copy, Clone)]
pub struct V<T: Read<usize>>(pub T);

/// We should be able to the value of the register from the state
impl<T: Read<usize>> Read<u8> for V<T> {
    fn read(&self, state: &State) -> u8 {
        state.registers.v_registers[self.0.read(state)]
    }
}

impl<'a, T: Read<usize>> Write<'a, u8> for V<T> {
    fn write(&self, state: &'a mut State) -> &'a mut u8 {
        &mut state.registers.v_registers[self.0.read(state)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, tribble::B12};

    #[test]
    fn test_read_v_register() {
        let mut state = State::new(&[]);
        state.registers.v_registers[3] = 4;
        assert_eq!(V(B4(3)).read(&state), 4);
    }

    #[test]
    #[should_panic]
    fn test_v_register_out_of_range() {
        let state = State::new(&[]);
        V(B12(B4(0b1111), B4(0b1111), B4(0b0000))).read(&state);
    }

    #[test]
    fn test_write_v_register() {
        let mut state = State::new(&[]);
        *V(B4(10)).write(&mut state) = 2;
        assert_eq!(state.registers.v_registers[10], 2);
    }
}
