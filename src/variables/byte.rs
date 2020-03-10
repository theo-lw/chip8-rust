use super::nibble::b4;
use super::Read;
use super::State;

/// Convenience class that represents an unsigned 8-bit number (aka, a byte)
/// First argument should be the upper half of the byte, the second should be the lower half
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct b8(pub b4, pub b4);

/// Creates a b8 from a u8
impl From<u8> for b8 {
    fn from(val: u8) -> b8 {
        let (upper, lower) = b4::from_u8(val);
        b8(upper, lower)
    }
}

/// Restores the u8 representation from a b8
impl Read<u8> for b8 {
    fn read(&self, _: &State) -> u8 {
        let b4(x) = self.0;
        let b4(y) = self.1;
        (x << 4) + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let val: u8 = 0b1001_1110;
        let byte = b8::from(val);
        assert_eq!(byte, b8(b4(0b1001), b4(0b1110)));
        let val: u8 = 0b0000_0000;
        let byte = b8::from(val);
        assert_eq!(byte, b8(b4(0b0000), b4(0b0000)));
    }

    #[test]
    fn test_read_u8() {
        let byte = b8(b4(0b1010), b4(0b0001));
        let state = State::new(&[]);
        assert_eq!(byte.read(&state), 0b1010_0001);
    }
}
