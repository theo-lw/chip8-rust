mod ret;
mod sys;

use super::chip8::State;

pub trait Instruction {
    fn execute(&self, state: &mut State);
}

pub fn parse() {}
