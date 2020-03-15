use super::{Instruction, State};
use crate::variables::{Read, Write};

/// Represents the SUB instruction (sets SUB.0 = SUB.0 + SUB.1)
/// Note that this does NOT set any flags on integer overflow
pub struct SUB<T, U>(pub T, pub U)
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>;

impl<T, U> Instruction for SUB<T, U>
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        let (result, carry): (u8, bool) = self.0.read(state).overflowing_sub(self.1.read(state));
        state.registers.v_registers[0xF] = u8::from(!carry);
        self.0.write(state, result);
    }
}

/// Represents the SUB instruction (sets SUB.0 = SUB.0 + SUB.1, sets VF = carry).
/// This is similar to the SUB struct. The difference is in how they handle integer overflow
pub struct SUBN<T, U>(pub T, pub U)
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>;

impl<T, U> Instruction for SUBN<T, U>
where
    T: Write<u8> + Read<u8>,
    U: Read<u8>,
{
    fn execute(&self, state: &mut State) {
        let (result, carry): (u8, bool) = self.1.read(state).overflowing_sub(self.0.read(state));
        state.registers.v_registers[0xF] = u8::from(!carry);
        self.0.write(state, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_sub() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[12] = 3;
        state.registers.v_registers[2] = 45;
        let sub = SUB(V(B4(2)), V(B4(12)));
        sub.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 3);
        assert_eq!(state.registers.v_registers[2], 42);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }

    #[test]
    fn test_subn() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[12] = 253;
        state.registers.v_registers[3] = 3;
        let subn = SUBN(V(B4(3)), V(B4(12)));
        subn.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 253);
        assert_eq!(state.registers.v_registers[3], 250);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }

    #[test]
    fn test_sub_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[12] = 200;
        state.registers.v_registers[11] = 45;
        let sub = SUB(V(B4(11)), V(B4(12)));
        sub.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 200);
        assert_eq!(state.registers.v_registers[11], 101);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }

    #[test]
    fn test_subn_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[4] = 3;
        state.registers.v_registers[7] = 252;
        let subn = SUBN(V(B4(7)), V(B4(4)));
        subn.execute(&mut state);
        assert_eq!(state.registers.v_registers[4], 3);
        assert_eq!(state.registers.v_registers[7], 7);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }
}
