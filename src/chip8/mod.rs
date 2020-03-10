mod display;
mod memory;
mod registers;
mod stack;
mod timers;

use memory::Memory;
use memory::PROGRAM_START;
use registers::Registers;
use stack::Stack;
use timers::Timers;

/// Struct representing the state of a Chip-8 machine
pub struct State {
    pub ram: Memory,
    pub registers: Registers,
    pub timers: Timers,
    pub stack: Stack,
    pub program_counter: usize,
}

impl State {
    /// Creates a new State struct
    pub fn new(program: &[u8]) -> State {
        State {
            ram: Memory::new(program),
            registers: Registers::new(),
            timers: Timers::new(),
            stack: Stack::new(),
            program_counter: PROGRAM_START,
        }
    }
}
