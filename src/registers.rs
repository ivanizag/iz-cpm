
// 8 bit registers
#[derive(Copy, Clone)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F, // Flags
    H,
    L,
    _HL_ // Not a real register
}
pub const REG_COUNT8: usize = 8;


// 16 bit registers
#[derive(Copy, Clone)]
pub enum Register16 {
    AF, // TODO: get from A, F
    BC,
    DE,
    HL,
    SP,
    PC
}
pub const REG_COUNT16: usize = 6;

// Flags, see http://www.z80.info/z80sflag.htm
#[derive(Copy, Clone)]
pub enum Flag {
    C  = 1,
    N  = 2,
    P  = 4,
    _3 = 8,
    H  = 16,
    _5 = 32,
    Z  = 64,
    S  = 128
}


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

    pub fn get_flag(&self, flag: &Flag) -> bool {
        self.get8(&Register8::F) & *flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: &Flag) {
        self.bytes[Register8::F as usize] |= *flag as u8;
    }

    pub fn clear_flag(&mut self, flag: &Flag) {
        self.bytes[Register8::F as usize] &= !(*flag as u8);
    }

    pub fn put_flag(&mut self, flag: &Flag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn update_sz53_flags(&mut self, reference: u8) {
        let f: &mut u8 = &mut self.bytes[Register8::F as usize];

        // Zero
        if reference == 0 {
            *f &= !(Flag::Z as u8)
        } else {
            *f |= Flag::Z as u8
        }

        // Bits 7, 5, and 3 are copied
        const MASK_S53: u8 = Flag::S as u8 + Flag::_5 as u8 + Flag::_3 as u8;
        *f = (*f & !MASK_S53) + (reference & MASK_S53);
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

    #[test]
    fn set_get_flag() {
        let mut r = Registers::new();
 
        assert_eq!(false, r.get_flag(&Flag::P));
        r.set_flag(&Flag::P);
        assert_eq!(true, r.get_flag(&Flag::P));
        r.clear_flag(&Flag::P);
        assert_eq!(false, r.get_flag(&Flag::P));
        r.put_flag(&Flag::P, true);
        assert_eq!(true, r.get_flag(&Flag::P));
        r.put_flag(&Flag::P, false);
        assert_eq!(false, r.get_flag(&Flag::P));
    }
}