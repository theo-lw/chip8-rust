use super::{Read, State, Write};

/// Struct representing the sound timer
pub struct ST;

/// We should be able to read a u8 from the sound timer
impl Read<u8> for ST {
    fn read(&self, state: &State) -> u8 {
        state.timers.sound_timer
    }
}

/// We should be able to write a u8 to the sound timer
impl Write<u8> for ST {
    fn write(&self, state: &mut State, val: u8) {
        state.timers.sound_timer = val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let st = ST;
        let mut state = State::mock(&[]);
        state.timers.sound_timer = 7;
        assert_eq!(st.read(&state), 7);
    }

    #[test]
    fn test_write_u8() {
        let st = ST;
        let mut state = State::mock(&[]);
        st.write(&mut state, 200);
        assert_eq!(state.timers.sound_timer, 200);
    }
}
