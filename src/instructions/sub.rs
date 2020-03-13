use super::{Instruction, State};
use crate::variables::{Read, Write};
use std::marker::PhantomData;
use std::ops::Sub;

#[macro_use]
use crate::overflow_op;

/// Represents the SUB instruction (sets SUB.0 = SUB.0 + SUB.1)
/// Note that this does NOT set any flags on integer overflow
pub struct SUB<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: OverflowingSub,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> SUB<'a, S, T, U>
where
    S: OverflowingSub,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create SUB without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        SUB(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for SUB<'a, S, T, U>
where
    S: OverflowingSub,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        let (result, carry): (S, bool) = self.0.read(state).overflowing_sub(&self.1.read(state));
        state.registers.v_registers[0xF] = u8::from(!carry);
        *self.0.write(state) = result;
    }
}

/// Represents the SUB instruction (sets SUB.0 = SUB.0 + SUB.1, sets VF = carry).
/// This is similar to the SUB struct. The difference is in how they handle integer overflow
pub struct SUBN<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: OverflowingSub,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> SUBN<'a, S, T, U>
where
    S: OverflowingSub,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create SUBN without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        SUBN(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for SUBN<'a, S, T, U>
where
    S: OverflowingSub,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        let (result, carry): (S, bool) = self.1.read(state).overflowing_sub(&self.0.read(state));
        state.registers.v_registers[0xF] = u8::from(!carry);
        *self.0.write(state) = result;
    }
}

/// Trait for types that can perform overflowing subition
///
/// The first element in the returned tuple should be the result of
/// performing wrapped subition on the left-hand side and the right-hand side
///
/// The second element should be true if an overflow occurred, false otherwise
pub trait OverflowingSub: Sized + Sub<Output = Self> {
    fn overflowing_sub(&self, rhs: &Self) -> (Self, bool);
}

overflow_op!(OverflowingSub, overflowing_sub, u8);
overflow_op!(OverflowingSub, overflowing_sub, u16);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{iregister::I, nibble::B4, vregister::V};

    #[test]
    fn test_sub() {
        let mut state = State::new(&[]);
        state.registers.v_registers[12] = 3;
        state.registers.i_register = 45;
        let sub = SUB::new(I, V(B4(12)));
        sub.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 3);
        assert_eq!(state.registers.i_register, 42);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }

    #[test]
    fn test_subn() {
        let mut state = State::new(&[]);
        state.registers.v_registers[12] = 253;
        state.registers.v_registers[3] = 3;
        let subn = SUBN::new(V(B4(3)), V(B4(12)));
        subn.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 253);
        assert_eq!(state.registers.v_registers[3], 250);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }

    #[test]
    fn test_sub_overflow() {
        let mut state = State::new(&[]);
        state.registers.v_registers[12] = 200;
        state.registers.v_registers[11] = 45;
        let sub = SUB::new(V(B4(11)), V(B4(12)));
        sub.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 200);
        assert_eq!(state.registers.v_registers[11], 101);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }

    #[test]
    fn test_subn_overflow() {
        let mut state = State::new(&[]);
        state.registers.v_registers[4] = 3;
        state.registers.v_registers[7] = 252;
        let subn = SUBN::new(V(B4(7)), V(B4(4)));
        subn.execute(&mut state);
        assert_eq!(state.registers.v_registers[4], 3);
        assert_eq!(state.registers.v_registers[7], 7);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }
}
