
// 8 bit registers, indexes in table "r" of http://www.z80.info/decoding.htm
pub const REG_B: usize = 0;
pub const REG_C: usize = 1;
pub const REG_D: usize = 2;
pub const REG_E: usize = 3;
pub const REG_H: usize = 4;
pub const REG_L: usize = 5;
// / is unused, it's (HL) for decoding
pub const REG_A: usize = 7;
pub const REG_COUNT8: usize = 14;

// 16 bit registers
pub const REG_BC: usize = 0;
pub const REG_DE: usize = 1;
pub const REG_HL: usize = 2;
pub const REG_SP: usize = 3;
pub const REG_PC: usize = 09;
pub const REG_COUNT16: usize = 14;

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

    pub fn get8(&self, reg: usize) -> u8 {
        self.bytes[reg]
    }

    pub fn set8(&mut self, reg: usize, value: u8) {
        self.bytes[reg] = value;
    }

    pub fn get16(&self, reg: usize) -> u16 {
        self.words[reg]
    }

    pub fn set16(&mut self, reg: usize, value: u16) {
        self.words[reg] = value;
    }

    pub fn get_a(&self) -> u8 {
        self.get8(REG_A)
    }

    pub fn set_a(&mut self, value: u8) {
        self.set8(REG_A, value);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut r = Registers::new();
        const V:u8 = 23;

        r.set8(REG_A, V);
        assert_eq!(V, r.get8(REG_A));
    }

    #[test]
    fn set_get_a() {
        let mut r = Registers::new();
        const V:u8 = 44;

        r.set_a(V);
        assert_eq!(V, r.get_a());
    }


}