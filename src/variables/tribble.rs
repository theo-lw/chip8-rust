use super::nibble::B4;
use super::Read;
use super::State;

/// Struct that represents a 12-bit unsigned number (aka, a tribble)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct B12(pub B4, pub B4, pub B4);

/// Conversion into u16
impl From<B12> for u16 {
    fn from(val: B12) -> u16 {
        (u16::from(u8::from(val.0)) << 8)
            + (u16::from(u8::from(val.1)) << 4)
            + u16::from(u8::from(val.2))
    }
}

/// We should be able to read a usize from a B12 using the conversion
impl Read<usize> for B12 {
    fn read(&self, _: &State) -> usize {
        usize::from(u16::from(*self))
    }
}

/// We should be able to read a u16 from a B12 using the conversion
impl Read<u16> for u16 {
    fn read(&self, _: &State) -> u16 {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_usize() {
        let val = B12(B4(0b1111), B4(0b1010), B4(0b1011));
        let state = State::new(&[]);
        assert_eq!(val.read(&state), 0b1111_1010_1011);
    }

    #[test]
    fn test_into_u16() {
        let val = B12(B4(0b1100), B4(0b0001), B4(0b0100));
        assert_eq!(u16::from(val), 0b1100_0001_0100);
    }

    #[test]
    fn test_read_u16() {
        let val = B12(B4(0b1001), B4(0b1000), B4(0b0000));
        let state = State::new(&[]);
        assert_eq!(val.read(&state), 0b1001_1000_0000);
    }
}
