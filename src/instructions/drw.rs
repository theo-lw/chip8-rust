use super::{Instruction, State};
use crate::chip8::display::Display;
use crate::variables::Read;

/// Represents the DRW instructions (draws DRW.2 bytes at position (DRW.0, DRW.1))
#[derive(Debug)]
pub struct DRW<S, T, U>(pub S, pub T, pub U)
where
    S: Read<usize>,
    T: Read<usize>,
    U: Read<usize>;

impl<S, T, U> Instruction for DRW<S, T, U>
where
    S: Read<usize>,
    T: Read<usize>,
    U: Read<usize>,
{
    fn execute(&self, state: &mut State) {
        let x = self.0.read(state);
        let y = self.1.read(state);
        let mut vf = 0;
        for i in 0..self.2.read(state) {
            let byte: u8 = state.memory.ram[usize::from(state.registers.i_register) + i];
            for j in 0..8 {
                let bit = (byte & (1 << (7 - j))) >> (7 - j);
                vf |= state
                    .display
                    .xor((x + j) % Display::WIDTH, (y + i) % Display::HEIGHT, bit);
            }
        }
        state.registers.v_registers[0xF] = vf;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_drw() {
        let mut state = State::mock(&[]);
        state.display.pixels[0][0] = 1;
        state.display.pixels[1][1] = 1;
        let drw = DRW(V(B4(0)), V(B4(1)), B4(5));
        drw.execute(&mut state);
        assert_eq!(state.registers.v_registers[0xF], 1);
        let image = [
            [0, 1, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 1, 0, 0, 0, 0],
            [1, 0, 0, 1, 0, 0, 0, 0],
            [1, 0, 0, 1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0, 0, 0, 0],
        ];
        for i in 0..5 {
            for j in 0..8 {
                assert_eq!(image[i][j], state.display.pixels[i][j]);
            }
        }
    }
}
