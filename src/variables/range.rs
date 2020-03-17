use super::{Read, State, Write};
use std::cmp::min;

pub struct RANGE<T>(pub Vec<T>);

impl<T, U> Read<Vec<T>> for RANGE<U>
where
    U: Read<T>,
{
    fn read(&self, state: &State) -> Vec<T> {
        self.0.iter().map(|x| x.read(state)).collect()
    }
}

impl<T, U> Write<Vec<U>> for RANGE<T>
where
    U: Clone,
    T: Write<U>,
{
    fn write(&self, state: &mut State, val: Vec<U>) {
        for (key, val) in val.iter().enumerate().take(min(self.0.len(), val.len())) {
            self.0[key].write(state, val.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{iregister::I, memory_at::AT, nibble::B4, vregister::V};

    #[test]
    fn test_read_vec() {
        let mut state = State::mock(&[]);
        let entries = [3, 14, 7, 20];
        for (key, val) in entries.iter().enumerate() {
            state.registers.v_registers[key] = *val;
        }
        let v = RANGE((0u8..4).map(|x| V(B4(x))).collect::<Vec<V<B4>>>());
        let result: Vec<u8> = v.read(&state);
        assert_eq!(result, entries.to_vec());
    }

    #[test]
    fn test_write_vec() {
        let mut state = State::mock(&[]);
        state.registers.i_register = 540;
        let v: Vec<AT<I>> = (0usize..5).map(|x| AT(I, x)).collect();
        let entries: Vec<u8> = vec![32, 44, 2, 9, 65];
        RANGE(v).write(&mut state, entries.clone());
        assert_eq!(state.memory.ram[540..545].to_vec(), entries);
    }
}
