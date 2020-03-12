/// Represents the RAM in Chip-8
pub struct Memory {
    pub ram: Vec<u8>,
}

impl Memory {
    /// Returns a Memory struct with sprites and a program loaded
    ///
    /// # Arguments
    ///
    /// * `program` - a vector of bytes representing the program to load
    pub fn new(program: &[u8]) -> Memory {
        let mut ram: Vec<u8> = vec![0; MAX_SIZE];
        let flat_sprites = SPRITES.iter().flatten().cloned().collect::<Vec<u8>>();
        ram.splice(..flat_sprites.len(), flat_sprites);
        ram.splice(
            PROGRAM_START..PROGRAM_START + program.len(),
            program.iter().cloned(),
        );
        Memory { ram: ram }
    }
}

/// Maximum size of the RAM in Chip-8
const MAX_SIZE: usize = 4096;

/// The starting index of a Chip-8 program
pub const PROGRAM_START: usize = 512;

/// The sprites stored in Chip-8 memory
const SPRITES: [[u8; 5]; 16] = [
    [
        0b1111_0000,
        0b1001_0000,
        0b1001_0000,
        0b1001_0000,
        0b1111_0000,
    ], // 0
    [
        0b0010_0000,
        0b0110_0000,
        0b0010_0000,
        0b0010_0000,
        0b0111_0000,
    ], // 1
    [
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
    ], // 2
    [
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
    ], // 3
    [
        0b1001_0000,
        0b1001_0000,
        0b1111_0000,
        0b0001_0000,
        0b0001_0000,
    ], // 4
    [
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
    ], // 5
    [
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
    ], // 6
    [
        0b1111_0000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b0100_0000,
    ], // 7
    [
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
    ], // 8
    [
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
        0b0001_0000,
        0b1111_0000,
    ], // 9
    [
        0b1111_0000,
        0b1001_0000,
        0b1111_0000,
        0b1001_0000,
        0b1001_0000,
    ], // A
    [
        0b1110_0000,
        0b1001_0000,
        0b1110_0000,
        0b1001_0000,
        0b1110_0000,
    ], // B
    [
        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1111_0000,
    ], // C
    [
        0b1110_0000,
        0b1001_0000,
        0b1001_0000,
        0b1001_0000,
        0b1110_0000,
    ], // D
    [
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
    ], // E
    [
        0b1111_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
    ], // F
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that sprites are loaded correctly
    fn test_sprites() {
        let mem = Memory::new(&[]);
        let flat_sprites = SPRITES.iter().flatten().cloned().collect::<Vec<u8>>();
        for (index, element) in flat_sprites.iter().enumerate() {
            assert_eq!(*element, mem.ram[index]);
        }
    }

    #[test]
    /// Test that the program is loaded correctly
    fn test_program() {
        let program = &[5, 24, 32, 4, 16, 50];
        let mem = Memory::new(program);
        for (index, element) in program.iter().enumerate() {
            assert_eq!(*element, mem.ram[PROGRAM_START + index]);
        }
    }
}
