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
#[derive(Copy, Clone, Debug, PartialEq)]
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
    iff1: bool,
    iff2: bool,
    im: u8
}

impl Registers {
    pub fn new() -> Registers {
        //Init z80 registers (TUZD-2.4)
        let mut reg = Registers {
            data: [0; REG_COUNT8],
            shadow: [0; REG_COUNT8],
            pc: 0,
            iff1: false,
            iff2: false,
            im: 0
        };

        reg.set16(Reg16::AF, 0xffff);
        reg.set16(Reg16::SP, 0xffff);

        reg
    }

    pub fn get_a(&self) -> u8 {
        self.data[Reg8::A as usize]
    }

    pub fn set_a(&mut self, value: u8) {
        self.data[Reg8::A as usize] = value;
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

    pub fn inc_dec8(&mut self, reg: Reg8, inc: bool) -> u8 {
        let mut v = self.get8(reg);
        if inc {
            v = v.wrapping_add(1);
        } else {
            v = v.wrapping_sub(1);
        }
        self.set8(reg, v);
        v
    }

    pub fn get16(&self, rr: Reg16) -> u16 {
        let (h, l) = Registers::get_pair(rr);

        self.data[l] as u16
        + ((self.data[h] as u16) << 8)
    }

    pub fn set16(&mut self, rr: Reg16, value: u16) {
        let (h, l) = Registers::get_pair(rr);

        self.data[l] = value as u8;
        self.data[h] = (value >> 8) as u8;
    }

    pub fn inc_dec16(&mut self, rr: Reg16, inc: bool) -> u16 {
        let mut v = self.get16(rr);
        if inc {
            v = v.wrapping_add(1);
        } else {
            v = v.wrapping_sub(1);
        }
        self.set16(rr, v);
        v
    }

    pub fn swap(&mut self, rr: Reg16) {
        let (h, l) = Registers::get_pair(rr);

        let temp = self.data[l];
        self.data[l] = self.shadow[l];
        self.shadow[l] = temp;

        let temp = self.data[h];
        self.data[h] = self.shadow[h];
        self.shadow[h] = temp;
    }

    fn get_pair(rr: Reg16) -> (usize, usize) {
        match rr {
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

    pub fn update_sz53p_flags(&mut self, reference: u8) {
        self.update_sz53_flags(reference);
        self.update_p_flag(reference);
    }

    pub fn update_sz53_flags(&mut self, reference: u8) {
        self.update_53_flags(reference);

        let f: &mut u8 = &mut self.data[Reg8::F as usize];
        // Zero
        if reference == 0 {
            *f |= Flag::Z as u8
        } else {
            *f &= !(Flag::Z as u8)
        }

        // Sign is copied
        const MASK_S: u8 = Flag::S as u8;
        *f = (*f & !MASK_S) + (reference & MASK_S);
    }

    pub fn update_53_flags(&mut self, reference: u8) {
        let f: &mut u8 = &mut self.data[Reg8::F as usize];

        // Bits 5, and 3 are copied
        const MASK_53: u8 = Flag::_5 as u8 + Flag::_3 as u8;
        *f = (*f & !MASK_53) + (reference & MASK_53);
    }

    pub fn update_p_flag(&mut self, reference: u8) {
        let bits = reference.count_ones();
        self.put_flag(Flag::P, bits % 2 == 0);
    }

    pub fn update_vh_flags(&mut self, xored: u16) {
        let half_bit  = (xored >> 4 & 1) != 0;
        self.put_flag(Flag::H, half_bit);

        let carry_bit = (xored >> 8 & 1) != 0;
        let top_xor   = (xored >> 7 & 1) != 0;
        self.put_flag(Flag::P, carry_bit != top_xor); // As overflow flag
    }

    pub fn update_cvh_flags(&mut self, xored: u16) {
        let carry_bit = (xored >> 8 & 1) != 0;
        self.put_flag(Flag::C, carry_bit);

        self.update_vh_flags(xored);
    }

    pub fn update_ch_flags(&mut self, xored: u16) {
        let carry_bit = (xored >> 8 & 1) != 0;
        self.put_flag(Flag::C, carry_bit);

        let half_bit  = (xored >> 4 & 1) != 0;
        self.put_flag(Flag::H, half_bit);

    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_interrupts(&mut self, v: bool) {
        self.iff2 = v;
    }

    pub fn set_interrup_mode(&mut self, im: u8) {
        self.im = im;
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