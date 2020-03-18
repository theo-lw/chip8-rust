use super::{Read, State, Write};

/// Struct representing the delay timer
#[derive(Debug)]
pub struct DT;

/// We should be able to read a u8 from the delay timer
impl Read<u8> for DT {
    fn read(&self, state: &State) -> u8 {
        state.timers.delay_timer
    }
}

/// We should be able to write a u8 to the delay timer
impl Write<u8> for DT {
    fn write(&self, state: &mut State, val: u8) {
        state.timers.delay_timer = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let st = DT;
        let mut state = State::mock(&[]);
        state.timers.delay_timer = 7;
        assert_eq!(st.read(&state), 7);
    }

    #[test]
    fn test_write_u8() {
        let st = DT;
        let mut state = State::mock(&[]);
        st.write(&mut state, 200);
        assert_eq!(state.timers.delay_timer, 200);
    }
}
