use super::{Read, Write, State};

/// Struct representing the memory at a location
pub struct AT<T: Read<usize>> (T);

/// We should be able to read a u8 from memory 
impl<T: Read<usize>> Read<u8> for AT<T> {
    fn read(&self, state: &State) -> u8 {
        state.memory.ram[self.0.read(state)]
    }
}

/// We should be able to write a u8 to memory
impl<'a, T: Read<usize>> Write<'a, u8> for AT<T> {
    fn write(&self, state: &'a mut State) -> &'a mut u8 {
        let location = self.0.read(state);
        &mut state.memory.ram[location]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{
        iregister::I,
        tribble::B12,
        nibble::B4
    };

    #[test]
    fn test_read_u8() {
        let at = AT(B12(B4(0b1001), B4(0b0100), B4(0b0010)));
        let mut state = State::new(&[]);
        state.memory.ram[0b1001_0100_0010] = 43;
        assert_eq!(at.read(&state), 43);
    }

    #[test]
    fn test_write_u8() {
        let at = AT(I);
        let mut state = State::new(&[]);
        state.registers.i_register = 1403;
        *at.write(&mut state) = 76;
        assert_eq!(state.memory.ram[1403], 76);
    }
}
