use super::{Instruction, State};
use crate::overflow_op;
use crate::variables::{Read, Write};
use std::marker::PhantomData;
use std::ops::Add;

/// Represents the ADD instruction (sets ADD.0 = ADD.0 + ADD.1)
/// Note that this does NOT set any flags on integer overflow
pub struct ADD<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: OverflowingAdd,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> ADD<'a, S, T, U>
where
    S: OverflowingAdd,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create ADD without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        ADD(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for ADD<'a, S, T, U>
where
    S: OverflowingAdd,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        let (result, _): (S, bool) = self.0.read(state).overflowing_add(&self.1.read(state));
        *self.0.write(state) = result;
    }
}

/// Represents the ADD instruction (sets ADD.0 = ADD.0 + ADD.1, sets VF = carry).
/// This is similar to the ADD struct. The difference is in how they handle integer overflow
pub struct ADDF<'a, S, T, U>(T, U, PhantomData<&'a S>)
where
    S: OverflowingAdd,
    T: Write<'a, S> + Read<S>,
    U: Read<S>;

impl<'a, S, T, U> ADDF<'a, S, T, U>
where
    S: OverflowingAdd,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    /// Convenience constructor to let us create ADDF without typing PhantomData
    pub fn new(left: T, right: U) -> Self {
        ADDF(left, right, PhantomData)
    }
}

impl<'a, S, T, U> Instruction<'a> for ADDF<'a, S, T, U>
where
    S: OverflowingAdd,
    T: Write<'a, S> + Read<S>,
    U: Read<S>,
{
    fn execute(&self, state: &'a mut State) {
        let (result, carry): (S, bool) = self.0.read(state).overflowing_add(&self.1.read(state));
        state.registers.v_registers[0xF] = u8::from(carry);
        *self.0.write(state) = result;
    }
}

/// Trait for types that can perform overflowing addition
///
/// The first element in the returned tuple should be the result of
/// performing wrapped addition on the left-hand side and the right-hand side
///
/// The second element should be true if an overflow occurred, false otherwise
pub trait OverflowingAdd: Sized + Add<Output = Self> {
    fn overflowing_add(&self, rhs: &Self) -> (Self, bool);
}

overflow_op!(OverflowingAdd, overflowing_add, u8);
overflow_op!(OverflowingAdd, overflowing_add, u16);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{iregister::I, nibble::B4, vregister::V};

    #[test]
    fn test_add() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[12] = 3;
        state.registers.i_register = 45;
        let add = ADD::new(I, V(B4(12)));
        add.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 3);
        assert_eq!(state.registers.i_register, 48);
    }

    #[test]
    fn test_addf_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[12] = 3;
        state.registers.v_registers[3] = 253;
        let addf = ADDF::new(V(B4(3)), V(B4(12)));
        addf.execute(&mut state);
        assert_eq!(state.registers.v_registers[12], 3);
        assert_eq!(state.registers.v_registers[3], 0);
        assert_eq!(state.registers.v_registers[0xF], 1);
    }

    #[test]
    fn test_addf_no_overflow() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[4] = 3;
        state.registers.v_registers[7] = 252;
        let addf = ADDF::new(V(B4(7)), V(B4(4)));
        addf.execute(&mut state);
        assert_eq!(state.registers.v_registers[4], 3);
        assert_eq!(state.registers.v_registers[7], 255);
        assert_eq!(state.registers.v_registers[0xF], 0);
    }
}
