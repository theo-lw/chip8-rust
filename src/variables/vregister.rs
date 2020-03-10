use super::nibble::b4;
use super::tribble::b12;
use super::Read;
use super::State;

/// Struct representing the contents at a v-register
/// There are only 16 v-registers, numbered from 0..16
/// It is expected that this struct is constructed with a value
/// that returns a number in the range 0..16 when read.
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct v<T: Read<usize>>(pub T);

/// We should be able to the value of the register from the state
impl<T: Read<usize>> Read<u8> for v<T> {
    fn read(&self, state: &State) -> u8 {
        state.registers.v_registers[self.0.read(state)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_v_register() {
        let mut state = State::new(&[]);
        state.registers.v_registers[3] = 4;
        assert_eq!(v(b4(3)).read(&state), 4);
    }

    #[test]
    #[should_panic]
    fn test_v_register_out_of_range() {
        let state = State::new(&[]);
        v(b12(b4(0b1111), b4(0b1111), b4(0b0000))).read(&state);
    }
}
