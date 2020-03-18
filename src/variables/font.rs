use super::{Read, State};

/// Represents the location of the sprite for F.0
#[derive(Debug)]
pub struct F<T>(pub T)
where
    T: Read<u8>;

impl<T> Read<u16> for F<T>
where
    T: Read<u8>,
{
    fn read(&self, state: &State) -> u16 {
        u16::from(self.0.read(state)) * 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_read_f() {
        let mut state = State::mock(&[]);
        state.registers.v_registers[3] = 0xF;
        let f = F(V(B4(3)));
        let sprite: [u8; 5] = [
            0b1111_0000,
            0b1000_0000,
            0b1111_0000,
            0b1000_0000,
            0b1000_0000,
        ];
        let result = usize::from(f.read(&state));
        for i in 0..5 {
            assert_eq!(state.memory.ram[result + i], sprite[i]);
        }
    }
}
