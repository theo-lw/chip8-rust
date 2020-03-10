use super::Instruction;
use super::State;
use crate::variables::Read;

pub struct SYS<T: Read<usize>>(T);

impl<T: Read<usize>> Instruction for SYS<T> {
    fn execute(&self, state: &mut State) {
        state.program_counter = self.0.read(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::b4, tribble::b12};

    #[test]
    fn test_sys() {
        let mut state = State::new(&[]);
        let sys = SYS(b12(b4(0b1011), b4(0b1001), b4(0b0010)));
        sys.execute(&mut state);
        assert_eq!(state.program_counter, 0b1011_1001_0010);
    }
}
