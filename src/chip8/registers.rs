/// Struct representing the registers of a chip-8 machine
#[derive(Default)]
pub struct Registers {
    pub v_registers: [u8; 16],
    pub i_register: u16,
}

impl Registers {
    /// Creates a new Registers struct with all values set to 0
    pub fn new() -> Registers {
        Default::default()
    }
}
