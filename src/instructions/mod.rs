mod add;
mod and;
mod call;
mod drw;
mod helpers;
mod jp;
mod ld;
mod or;
mod ret;
mod rnd;
mod se;
mod shl;
mod shr;
mod sknp;
mod skp;
mod sne;
mod sub;
mod sys;
mod xor;

use super::chip8::State;

/// A trait for instructions.
///
/// Defines one method, `execute(&self, &mut State)` because
/// instructions should be able to be executed in the context of a State struct
pub trait Instruction {
    fn execute(&self, state: &mut State);
}

pub fn parse() {}
