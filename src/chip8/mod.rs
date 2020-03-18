pub mod display;
pub mod keyboard;
pub mod memory;
pub mod registers;
pub mod stack;
pub mod timers;

use display::Display;
use keyboard::Keyboard;
use memory::Memory;
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
    /// Creates a new State struct with no IO (no display, no keyboard).
    /// Used for testing purposes only!
    #[cfg(test)]
    pub fn mock(program: &[u8]) -> Self {
        use crate::config::Color;
        use keyboard::MockKeyboard;
        use memory::PROGRAM_START;
        State {
            display: Display::new(Color::white(), Color::black()),
            memory: Memory::new(program),
            registers: Registers::new(),
            timers: Timers::new(),
            stack: Stack::new(),
            program_counter: PROGRAM_START,
            keyboard: Box::new(MockKeyboard::new()),
        }
    }
}
