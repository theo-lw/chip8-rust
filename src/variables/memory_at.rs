use super::{Read, State, Write};

/// Struct representing the memory at a location at AT.0 + AT.1
/// The first field represents the starting position. The second represents the offset
pub struct AT<T: Read<usize>>(pub T, pub usize);

/// We should be able to read a u8 from memory
impl<T: Read<usize>> Read<u8> for AT<T> {
    fn read(&self, state: &State) -> u8 {
        state.memory.ram[self.0.read(state) + self.1]
    }
}

/// We should be able to write a u8 to memory
impl<T: Read<usize>> Write<u8> for AT<T> {
    fn write(&self, state: &mut State, val: u8) {
        let location: usize = self.0.read(state);
        state.memory.ram[location + self.1] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{iregister::I, nibble::B4, tribble::B12};

    #[test]
    fn test_read_u8() {
        let at = AT(B12(B4(0b1001), B4(0b0100), B4(0b0010)), 0);
        let mut state = State::mock(&[]);
        state.memory.ram[0b1001_0100_0010] = 43;
        assert_eq!(at.read(&state), 43);
    }

    #[test]
    fn test_write_u8() {
        let at = AT(I, 0);
        let mut state = State::mock(&[]);
        state.registers.i_register = 1403;
        at.write(&mut state, 76);
        assert_eq!(state.memory.ram[1403], 76);
    }
}
