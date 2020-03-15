use std::fmt;

// 8 bit registers
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F, // Flags
    H,
    L,
    I,
    R,
    IXH,
    IXL,
    IYH,
    IYL,
    SPH,
    SPL,
    _HL // Invalid
}
pub const REG_COUNT8: usize = 16;


// 16 bit registers, composed from 8 bit registers
#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    IX,
    IY,
    SP,
}

// Flags, see http://www.z80.info/z80sflag.htm
#[derive(Copy, Clone, Debug)]
pub enum Flag {
    C  = 1,
    N  = 2,
    P  = 4, // P/V
    _3 = 8,
    H  = 16,
    _5 = 32,
    Z  = 64,
    S  = 128
}

impl fmt::Display for Reg8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Reg8::_HL => write!(f, "(HL)"),
            _ => write!(f, "{:?}", *self)
        }
    }
}


#[derive(Debug)]
pub struct Registers {
    data: [u8; REG_COUNT8],
    shadow: [u8; REG_COUNT8],
    pc: u16,
    iff: bool
}

impl Registers {
    pub fn new() -> Registers {
        let mut reg = Registers {
            data: [0; REG_COUNT8],
            shadow: [0; REG_COUNT8],
            pc: 0,
            iff: true
        };

        //Init z80 registers
        reg.set16(Reg16::AF, 0xffff);
        reg.set16(Reg16::SP, 0xffff);

        reg
    }

    pub fn get8(&self, reg: Reg8) -> u8 {
        if reg == Reg8::_HL {
            panic!("Can't use the pseudo register (HL)");
        }
        self.data[reg as usize]
    }

    pub fn set8(&mut self, reg: Reg8, value: u8) {
        if reg == Reg8::_HL {
            panic!("Can't use the pseudo register (HL)");
        }
        self.data[reg as usize] = value;
    }

    pub fn get16(&self, reg: Reg16) -> u16 {
        let (h, l) = Registers::get_pair(reg);

        self.data[l] as u16
        + ((self.data[h] as u16) << 8)
    }

    pub fn set16(&mut self, reg: Reg16, value: u16) {
        let (h, l) = Registers::get_pair(reg);

        self.data[l] = value as u8;
        self.data[h] = (value >> 8) as u8;
    }

    pub fn swap(&mut self, reg: Reg16) {
        let (h, l) = Registers::get_pair(reg);

        let temp = self.data[l];
        self.data[l] = self.shadow[l];
        self.shadow[l] = temp;

        let temp = self.data[h];
        self.data[h] = self.shadow[h];
        self.shadow[h] = temp;
    }

    fn get_pair(reg: Reg16) -> (usize, usize) {
        match reg {
            Reg16::AF => (Reg8::A as usize, Reg8::F as usize),
            Reg16::BC => (Reg8::B as usize, Reg8::C as usize),
            Reg16::DE => (Reg8::D as usize, Reg8::E as usize),
            Reg16::HL => (Reg8::H as usize, Reg8::L as usize),
            Reg16::IX => (Reg8::IXH as usize, Reg8::IXL as usize),
            Reg16::IY => (Reg8::IYH as usize, Reg8::IYL as usize),
            Reg16::SP => (Reg8::SPH as usize, Reg8::SPL as usize)
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.get8(Reg8::F) & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.data[Reg8::F as usize] |= flag as u8;
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.data[Reg8::F as usize] &= !(flag as u8);
    }

    pub fn put_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn update_sz53_flags(&mut self, reference: u8) {
        let f: &mut u8 = &mut self.data[Reg8::F as usize];

        // Zero
        if reference == 0 {
            *f |= Flag::Z as u8
        } else {
            *f &= !(Flag::Z as u8)
        }

        // Bits 7, 5, and 3 are copied
        const MASK_S53: u8 = Flag::S as u8 + Flag::_5 as u8 + Flag::_3 as u8;
        *f = (*f & !MASK_S53) + (reference & MASK_S53);
    }

    pub fn update_p_flag(&mut self, reference: u8) {
        let bits = reference.count_ones();
        self.put_flag(Flag::P, bits % 2 == 0);
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_interrupts(&mut self, v: bool) {
        self.iff = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut r = Registers::new();
        const V:u8 = 23;

        r.set8(Reg8::A, V);
        assert_eq!(V, r.get8(Reg8::A));
    }

    #[test]
    fn set_get_16bit_register() {
        let mut r = Registers::new();

        r.set16(Reg16::BC, 0x34de);
        assert_eq!(0x34de, r.get16(Reg16::BC));
        assert_eq!(0x34, r.get8(Reg8::B));
        assert_eq!(0xde, r.get8(Reg8::C));
    }

    #[test]
    fn set_get_flag() {
        let mut r = Registers::new();
 
        r.set_flag(Flag::P);
        assert_eq!(true, r.get_flag(Flag::P));
        r.clear_flag(Flag::P);
        assert_eq!(false, r.get_flag(Flag::P));
        r.put_flag(Flag::P, true);
        assert_eq!(true, r.get_flag(Flag::P));
        r.put_flag(Flag::P, false);
        assert_eq!(false, r.get_flag(Flag::P));
    }
}