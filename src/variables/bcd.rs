use super::{Read, State};

#[derive(Debug)]
pub struct BCD<T>(pub T)
where
    T: Read<u8>;

impl<T> Read<Vec<u8>> for BCD<T>
where
    T: Read<u8>,
{
    fn read(&self, state: &State) -> Vec<u8> {
        let mut val: u8 = self.0.read(state);
        let ones = val % 10;
        val /= 10;
        let tens = val % 10;
        val /= 10;
        let hundreds = val % 10;
        vec![hundreds, tens, ones]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::byte::B8;

    #[test]
    fn test_bcd_read() {
        let state = State::mock(&[]);
        let bcd = BCD(B8::from(231));
        assert_eq!(bcd.read(&state), vec![2, 3, 1]);
        let bcd = BCD(B8::from(31));
        assert_eq!(bcd.read(&state), vec![0, 3, 1]);
    }
}
