use super::chip8::State;

pub trait Read<T> {
    fn read(from: &State) -> T;
}

pub trait Write<T> {
    fn write(to: &State, from: T);
}
