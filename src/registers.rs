
// 8 bit registers
#[derive(Copy, Clone)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    _HL_ // Not a real register
}
pub const REG_COUNT8: usize = 8;


// 16 bit registers
#[derive(Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}
pub const REG_COUNT16: usize = 6;

#[derive(Debug)]
pub struct Registers {
    bytes: [u8; REG_COUNT8],
    words: [u16; REG_COUNT16]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            bytes: [0; REG_COUNT8],
            words: [0; REG_COUNT16]
        }
    }

    pub fn get8(&self, reg: &Register8) -> u8 {
        self.bytes[*reg as usize]
    }

    pub fn set8(&mut self, reg: &Register8, value: u8) {
        self.bytes[*reg as usize] = value;
    }

    pub fn get16(&self, reg: &Register16) -> u16 {
        self.words[*reg as usize]
    }

    pub fn set16(&mut self, reg: &Register16, value: u16) {
        self.words[*reg as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut r = Registers::new();
        const V:u8 = 23;

        r.set8(&Register8::A, V);
        assert_eq!(V, r.get8(&Register8::A));
    }
}