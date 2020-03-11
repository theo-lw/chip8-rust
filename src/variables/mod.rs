pub mod byte;
pub mod iregister;
pub mod nibble;
pub mod tribble;
pub mod vregister;

use super::chip8::State;

/// Trait for variables that can be read from chip-8's state
pub trait Read<T> {
    fn read(&self, from: &State) -> T;
}

/// Trait for variables that can be written to chip-8's state
pub trait Write<T> {
    fn write(&self, to: State) -> &mut T;
}
