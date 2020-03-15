mod display;
mod keyboard;
mod memory;
mod registers;
mod stack;
mod timers;

use display::Display;
use keyboard::Keyboard;
#[cfg(test)]
use keyboard::MockKeyboard;
use memory::Memory;
#[cfg(test)]
use memory::PROGRAM_START;
use registers::Registers;
use stack::Stack;
use timers::Timers;

/// Struct representing the state of a Chip-8 machine
pub struct State {
    pub display: Display,
    pub memory: Memory,
    pub registers: Registers,
    pub timers: Timers,
    pub stack: Stack,
    pub program_counter: usize,
    pub keyboard: Box<dyn Keyboard>,
}

impl State {
    /// Creates a new State struct
    #[cfg(test)]
    pub fn new(program: &[u8]) -> Self {
        State {
            display: Display::new(vec![]),
            memory: Memory::new(program),
            registers: Registers::new(),
            timers: Timers::new(),
            stack: Stack::new(),
            program_counter: PROGRAM_START,
            keyboard: Box::new(MockKeyboard::new()),
        }
    }
}
