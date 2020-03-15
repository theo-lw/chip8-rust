use super::{Instruction, State};
use crate::variables::Read;

/// Represents the DRW instructions (draws DRW.2 bytes at position (DRW.0, DRW.1))
pub struct DRW<S, T, U>(pub S, pub T, pub U)
where
    S: Read<usize>,
    T: Read<usize>,
    U: Read<usize>;

impl<'a, S, T, U> Instruction<'a> for DRW<S, T, U>
where
    S: Read<usize>,
    T: Read<usize>,
    U: Read<usize>,
{
    fn execute(&self, state: &'a mut State) {
        let x = self.0.read(state);
        let y = self.1.read(state);
        let mut vf = false;
        for i in 0..self.2.read(state) {
            let byte: u8 = state.memory.ram[usize::from(state.registers.i_register) + i];
            for j in 0..8 {
                let bit = (byte & (1 << (7 - j))) >> (7 - j);
                vf |= state.display.xor(x + j, y + i, bit != 0);
            }
        }
        state.registers.v_registers[0xF] = u8::from(vf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, vregister::V};

    #[test]
    fn test_drw_no_collision() {
        let mut state = State::mock(&[]);
        let drw = DRW(V(B4(0)), V(B4(1)), B4(5));
        drw.execute(&mut state);
        assert_eq!(state.registers.v_registers[0xF], 0);
        let image = [
            [true, true, true, true, false, false, false, false],
            [true, false, false, true, false, false, false, false],
            [true, false, false, true, false, false, false, false],
            [true, false, false, true, false, false, false, false],
            [true, true, true, true, false, false, false, false],
        ];
        for i in 0..5 {
            for j in 0..8 {
                assert_eq!(image[i][j], state.display.pixels[i][j]);
            }
        }
    }
}
