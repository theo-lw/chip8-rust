use crate::config::Color;

/// Struct representing the display of a chip-8 machine
pub struct Display {
    inactive_color: Color,
    active_color: Color,
    pub colored_pixels: [u8; Self::WIDTH * Self::HEIGHT * 4],
    pub pixels: [[u8; Self::WIDTH]; Self::HEIGHT],
}

impl Display {
    /// The width of the display (in pixels)
    pub const WIDTH: usize = 64;

    /// The height of the display (in pixels)
    pub const HEIGHT: usize = 32;

    /// Creates a new display with no active pixels
    pub fn new(active_color: Color, inactive_color: Color) -> Self {
        let mut result = Display {
            active_color,
            inactive_color,
            colored_pixels: [0; Self::WIDTH * Self::HEIGHT * 4],
            pixels: [[0; Self::WIDTH]; Self::HEIGHT],
        };
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                result.set_colored_pixel(x, y, inactive_color);
            }
        }
        result
    }

    /// XOR's a pixel at (x, y) onto the screen.
    /// Returns true if the pixel at (x, y) is turned from active to inactive
    pub fn xor(&mut self, x: usize, y: usize, val: u8) -> u8 {
        let result = self.pixels[y][x] & val;
        self.pixels[y][x] ^= val;
        let color: Color = if self.pixels[y][x] == 1 {
            self.active_color
        } else {
            self.inactive_color
        };
        self.set_colored_pixel(x, y, color);
        result
    }

    fn set_colored_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.colored_pixels[(Self::WIDTH * y + x) * 4] = color.0;
        self.colored_pixels[(Self::WIDTH * y + x) * 4 + 1] = color.1;
        self.colored_pixels[(Self::WIDTH * y + x) * 4 + 2] = color.2;
        self.colored_pixels[(Self::WIDTH * y + x) * 4 + 3] = color.3;
    }

    /// Clears the display of all pixels
    pub fn clear(&mut self) {
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                self.pixels[y][x] = 0;
                self.set_colored_pixel(x, y, self.inactive_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_xor() {
        let mut display = Display::new(Color::white(), Color::black());
        assert_eq!(display.xor(46, 21, 0), 0);
        assert_eq!(display.pixels[21][46], 0);
        assert_eq!(display.xor(46, 21, 1), 0);
        assert_eq!(display.pixels[21][46], 1);
        assert_eq!(display.xor(46, 21, 0), 0);
        assert_eq!(display.pixels[21][46], 1);
        assert_eq!(display.xor(46, 21, 1), 1);
        assert_eq!(display.pixels[21][46], 0);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::new(Color::white(), Color::black());
        display.pixels[3][4] = 1;
        display.pixels[20][31] = 1;
        display.pixels[6][1] = 1;
        display.clear();
        for row in display.pixels.iter() {
            for pixel in row.iter() {
                assert_eq!(*pixel, 0);
            }
        }
    }
}
