use super::Instruction;
use super::State;

/// Represents the RET instruction (returns from subroutine)
pub struct RET;

impl<'a> Instruction<'a> for RET {
    fn execute(&self, state: &mut State) {
        state.program_counter = state.stack.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ret() {
        let mut state = State::new(&[]);
        state.stack.push(3).unwrap();
        let ret = RET;
        ret.execute(&mut state);
        assert_eq!(state.program_counter, 3);
        assert_eq!(state.stack.top(), None);
    }

    #[test]
    #[should_panic]
    fn test_ret_empty_stack() {
        let mut state = State::new(&[]);
        let ret = RET;
        ret.execute(&mut state);
    }
}
