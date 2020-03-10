use super::Read;
use super::State;

/// Struct that represents an unsigned 4-bit number (aka, nibble)
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct b4(pub u8);

impl b4 {
    /// Takes a u8 and returns (upper nibble, lower nibble)
    pub fn from_u8(val: u8) -> (b4, b4) {
        (b4(val >> 4), b4((val << 4) >> 4))
    }
}

/// Conversion into usize
impl From<b4> for usize {
    fn from(val: b4) -> Self {
        usize::from(val.0)
    }
}

/// We should be able to read a usize from a b4 using the conversion
impl Read<usize> for b4 {
    fn read(&self, _: &State) -> usize {
        usize::from(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let (b4(x), b4(y)) = b4::from_u8(0b1010_1100);
        assert_eq!(x, 0b1010);
        assert_eq!(y, 0b1100);
        let (b4(x), b4(y)) = b4::from_u8(0b1111_0000);
        assert_eq!(x, 0b1111);
        assert_eq!(y, 0b0000);
    }

    #[test]
    fn test_read_usize() {
        let state = State::new(&[]);
        let (x, y): (b4, b4) = b4::from_u8(0b0110_1001);
        assert_eq!(x.read(&state), 0b0110);
        assert_eq!(y.read(&state), 0b1001);
    }
}
