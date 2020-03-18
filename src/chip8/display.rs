/// Struct representing the display of a chip-8 machine
pub struct Display {
    observers: Vec<Box<dyn FnMut(DisplayEvent)>>,
    pub pixels: [[bool; Self::WIDTH]; Self::HEIGHT],
}

/// Enum representing the events that can be emitted by the display
#[derive(Clone, Copy, Debug)]
pub enum DisplayEvent {
    XOR(usize, usize, bool),
    CLEAR,
    PRESENT,
}

impl Display {
    /// The width of the display (in pixels)
    pub const WIDTH: usize = 64;

    /// The height of the display (in pixels)
    pub const HEIGHT: usize = 32;

    /// Creates a new display with no active pixels
    pub fn new(observers: Vec<Box<dyn FnMut(DisplayEvent)>>) -> Self {
        Display {
            observers,
            pixels: [[false; Self::WIDTH]; Self::HEIGHT],
        }
    }

    /// Notifies the observers
    fn notify_observers(&mut self, event: DisplayEvent) {
        for f in self.observers.iter_mut() {
            f(event);
        }
    }

    /// XOR's a pixel at (x, y) onto the screen.
    /// Returns true if the pixel at (x, y) is turned from active to inactive
    pub fn xor(&mut self, x: usize, y: usize, val: bool) -> bool {
        let result = self.pixels[y][x] & val;
        self.pixels[y][x] ^= val;
        // if val is true, then a the pixel must have changed, so we notify the observers
        if val {
            self.notify_observers(DisplayEvent::XOR(x, y, self.pixels[y][x]));
        }
        result
    }

    /// Clears the display of all pixels
    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = false;
            }
        }
        self.notify_observers(DisplayEvent::CLEAR);
    }

    /// Presents the display
    pub fn present(&mut self) {
        self.notify_observers(DisplayEvent::PRESENT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_xor() {
        let mut display = Display::new(vec![]);
        assert_eq!(display.xor(46, 21, false), false);
        assert_eq!(display.pixels[21][46], false);
        assert_eq!(display.xor(46, 21, true), false);
        assert_eq!(display.pixels[21][46], true);
        assert_eq!(display.xor(46, 21, false), false);
        assert_eq!(display.pixels[21][46], true);
        assert_eq!(display.xor(46, 21, true), true);
        assert_eq!(display.pixels[21][46], false);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::new(vec![]);
        display.pixels[3][4] = true;
        display.pixels[20][31] = true;
        display.pixels[6][1] = true;
        display.clear();
        for row in display.pixels.iter() {
            for pixel in row.iter() {
                assert_eq!(*pixel, false);
            }
        }
    }
}
