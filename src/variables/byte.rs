use super::nibble::B4;
use super::Read;
use super::State;

/// Convenience class that represents an unsigned 8-bit number (aka, a byte)
/// First argument should be the upper half of the byte, the second should be the lower half
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct B8(pub B4, pub B4);

/// Creates a B8 from a u8
impl From<u8> for B8 {
    fn from(val: u8) -> B8 {
        let (upper, lower) = B4::from_u8(val);
        B8(upper, lower)
    }
}

/// Restores the u8 representation from a B8
impl Read<u8> for B8 {
    fn read(&self, _: &State) -> u8 {
        let B4(x) = self.0;
        let B4(y) = self.1;
        (x << 4) + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let val: u8 = 0b1001_1110;
        let byte = B8::from(val);
        assert_eq!(byte, B8(B4(0b1001), B4(0b1110)));
        let val: u8 = 0b0000_0000;
        let byte = B8::from(val);
        assert_eq!(byte, B8(B4(0b0000), B4(0b0000)));
    }

    #[test]
    fn test_read_u8() {
        let byte = B8(B4(0b1010), B4(0b0001));
        let state = State::new(&[]);
        assert_eq!(byte.read(&state), 0b1010_0001);
    }
}
