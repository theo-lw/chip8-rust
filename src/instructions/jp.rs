use super::{Instruction, State};
use crate::variables::Read;

/// Represents the JP instruction (jump to the location at JP.0 + JP.1)
#[derive(Debug)]
pub struct JP<T, U>(pub T, pub U)
where
    T: Read<usize>,
    U: Read<usize>;

impl<T, U> Instruction for JP<T, U>
where
    T: Read<usize>,
    U: Read<usize>,
{
    fn execute(&self, state: &mut State) {
        state.program_counter = self.1.read(state).wrapping_add(self.0.read(state)) - 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, tribble::B12};

    #[test]
    fn test_jp() {
        let mut state = State::mock(&[]);
        let jp = JP(B4(0), B12(B4(0b0000), B4(0b1001), B4(0b0010)));
        jp.execute(&mut state);
        state.program_counter += 2;
        assert_eq!(state.program_counter, 0b0000_1001_0010);
    }
}
