use super::chip8::State;

pub trait Instruction {
    fn evaluate(&self, state: &mut State);
}
