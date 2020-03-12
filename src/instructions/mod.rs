mod call;
mod jp;
mod ret;
mod se;
mod sys;

use super::chip8::State;

/// A trait for instructions.
///
/// Defines one method, `execute(&self, &mut State)` because
/// instructions should be able to be executed in the context of a State struct
pub trait Instruction {
    fn execute(&self, state: &mut State);
}

pub fn parse() {}
