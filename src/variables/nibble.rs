use super::Read;
use super::State;

/// Struct that represents an unsigned 4-bit number (aka, nibble)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct B4(pub u8);

impl B4 {
    /// Takes a u8 and returns (upper nibble, lower nibble)
    pub fn from_u8(val: u8) -> (B4, B4) {
        (B4(val >> 4), B4((val << 4) >> 4))
    }
}

/// Conversion into u8
impl From<B4> for u8 {
    fn from(val: B4) -> Self {
        val.0
    }
}

/// We should be able to read a usize from a B4 using the conversion
impl Read<usize> for B4 {
    fn read(&self, _: &State) -> usize {
        usize::from(u8::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let (B4(x), B4(y)) = B4::from_u8(0b1010_1100);
        assert_eq!(x, 0b1010);
        assert_eq!(y, 0b1100);
        let (B4(x), B4(y)) = B4::from_u8(0b1111_0000);
        assert_eq!(x, 0b1111);
        assert_eq!(y, 0b0000);
    }

    #[test]
    fn test_read_usize() {
        let state = State::mock(&[]);
        let (x, y): (B4, B4) = B4::from_u8(0b0110_1001);
        assert_eq!(x.read(&state), 0b0110);
        assert_eq!(y.read(&state), 0b1001);
    }

    #[test]
    fn test_into_u8() {
        let val = B4(0b0010);
        assert_eq!(u8::from(val), 0b0010);
    }
}
