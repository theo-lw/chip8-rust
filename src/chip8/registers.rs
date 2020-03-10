/// Struct representing the registers of a chip-8 machine
pub struct Registers {
    pub v_registers: [u8; 16],
    pub i_register: u16,
}

impl Registers {
    /// Creates a new Registers struct with all values set to 0
    pub fn new() -> Registers {
        Registers {
            v_registers: [0; 16],
            i_register: 0,
        }
    }
}
