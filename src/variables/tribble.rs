use super::nibble::b4;
use super::Read;
use super::State;

/// Struct that represents a 12-bit unsigned number (aka, a tribble)
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub struct b12(pub b4, pub b4, pub b4);

/// Conversion into usize
impl From<b12> for usize {
    fn from(val: b12) -> usize {
        (usize::from(val.0) << 8) + (usize::from(val.1) << 4) + usize::from(val.2)
    }
}

/// We should be able to read a usize from a b4 using the conversion
impl Read<usize> for b12 {
    fn read(&self, _: &State) -> usize {
        usize::from(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usize_read() {
        let val = b12(b4(0b1111), b4(0b1010), b4(0b1011));
        let state = State::new(&[]);
        assert_eq!(val.read(&state), 0b1111_1010_1011);
    }
}
