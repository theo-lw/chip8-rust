/// Struct representing the display of a chip-8 machine
pub struct Display {
    pub pixels: [[bool; 64]; 32],
}

impl Display {
    /// Creates a new display with no active pixels
    pub fn new() -> Display {
        Display {
            pixels: [[false; 64]; 32],
        }
    }

    /// XOR's a pixel at (x, y) onto the screen.
    /// Returns true if the pixel at (x, y) is turned from active to inactive
    pub fn xor(&mut self, x: usize, y: usize, val: bool) -> bool {
        let result = self.pixels[y][x] & val;
        self.pixels[y][x] ^= val;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_xor() {
        let mut display = Display::new();
        assert_eq!(display.xor(46, 21, false), false);
        assert_eq!(display.pixels[21][46], false);
        assert_eq!(display.xor(46, 21, true), false);
        assert_eq!(display.pixels[21][46], true);
        assert_eq!(display.xor(46, 21, false), false);
        assert_eq!(display.pixels[21][46], true);
        assert_eq!(display.xor(46, 21, true), true);
        assert_eq!(display.pixels[21][46], false);
    }
}
