use super::opcode::*;
use super::state::*;
use super::registers::*;

#[derive(Copy, Clone)]
pub enum ShiftMode {
    Arithmetic,
    Logical,
    Rotate,
    RotateCarry
}

#[derive(Copy, Clone)]
pub enum ShiftDir {
    Left,
    Right
}

// TODO: special cases for DDCB and FDCB prefix
pub fn build_rot_r(r: Reg8, (dir, mode, name): (ShiftDir, ShiftMode, &str), fast: bool) -> Opcode {
    let separator = if fast {""} else {" "};
    Opcode {
        name: format!("{}{}{}", name, separator, r),
        cycles: if fast {4} else {8}, // The one byte opcodes are faster // (HL): 15, (IX+d): 23
        action: Box::new(move |state: &mut State| {
            let mut v = state.get_reg(r);
            let carry: bool;

            match dir {
                ShiftDir::Left => {
                    let upper_bit = v >= 0x80;
                    v = v << 1;
                    let set_lower_bit = match mode {
                        ShiftMode::Arithmetic => false, // always 0 in bit 0
                        ShiftMode::Logical => true, // always 1 in bit 0
                        ShiftMode::Rotate => state.reg.get_flag(Flag::C), // carry in bit 0
                        ShiftMode::RotateCarry => upper_bit, // bit 7 moves to bit 0
                    };
                    if set_lower_bit { // bit 0 is 0 already
                        v = v | 1;
                    }
                    carry = upper_bit;
                },
                ShiftDir::Right => {
                    let upper_bit = v >= 0x80;
                    let lower_bit = (v & 1) == 1;
                    v = v >> 1;
                    let set_upper_bit = match mode {
                        ShiftMode::Arithmetic => upper_bit, // extend bit 7
                        ShiftMode::Logical => false, // always 0 in bit 7
                        ShiftMode::Rotate => state.reg.get_flag(Flag::C), // carry in bit 0
                        ShiftMode::RotateCarry => lower_bit, // bit 0 goes to bit 7
                    };
                    if set_upper_bit { // bit 7 is 0 already
                        v = v | 0x80;
                    }
                    carry = lower_bit;
                }
            }
            state.set_reg(r, v);
            state.reg.put_flag(Flag::C, carry);
            state.reg.clear_flag(Flag::H);
            state.reg.clear_flag(Flag::N);
            if fast {
                state.reg.update_53_flags(v);
            } else {
                state.reg.update_sz53p_flags(v);
            }
        })
    }
}

// TODO: special cases for DDCB and FDCB prefix
pub fn build_bit_r(bit: u8, r: Reg8) -> Opcode {
    Opcode {
        name: format!("BIT {}, {}", bit, r),
        cycles: 8, // (HL) 12, (IX+d) 20
        action: Box::new(move |state: &mut State| {
            let v8 = state.get_reg(r);
            let z = v8 & (1<<bit);
            state.reg.update_sz53p_flags(z); // TUZD-4.1, TOOD: exceptions for (HL)
        })
    }
}

// TODO: special cases for DDCB and FDCB prefix
pub fn build_set_r(bit: u8, r: Reg8) -> Opcode {
    Opcode {
        name: format!("SET {}, {}", bit, r),
        cycles: 8, // (HL) 15, (IX+d) 23
        action: Box::new(move |state: &mut State| {
            let mut v = state.get_reg(r);
            v = v | (1<<bit);
            state.set_reg(r, v);
        })
    }
}

// TODO: special cases for DDCB and FDCB prefix
pub fn build_res_r(bit: u8, r: Reg8) -> Opcode {
    Opcode {
        name: format!("RES {}, {}", bit, r),
        cycles: 8, // (HL) 15, (IX+d) 23
        action: Box::new(move |state: &mut State| {
            let mut v = state.get_reg(r);
            v = v & !(1<<bit);
            state.set_reg(r, v);
        })
    }
}

pub fn build_cpl() -> Opcode {
    Opcode {
        name: "CPL".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get_a();
            v = !v;
            state.reg.set_a(v);

            state.reg.set_flag(Flag::H);
            state.reg.set_flag(Flag::N);
        })
    }
}

pub fn build_scf() -> Opcode {
    Opcode {
        name: "SCF".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.reg.set_flag(Flag::C);
            state.reg.clear_flag(Flag::H);
            state.reg.clear_flag(Flag::N);
        })
    }
}

pub fn build_ccf() -> Opcode {
    Opcode {
        name: "SCF".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.reg.put_flag(Flag::C, !state.reg.get_flag(Flag::C));
            state.reg.put_flag(Flag::H, !state.reg.get_flag(Flag::H));
            state.reg.clear_flag(Flag::N);
        })
    }
}

pub fn build_rxd(dir: ShiftDir, name: &str) -> Opcode {
    Opcode {
        name: name.to_string(),
        cycles: 18,
        action: Box::new(move |state: &mut State| {
            let mut a = state.reg.get_a();
            let mut phl = state.get_reg(Reg8::_HL);
            // a = 0xWX, phl = 0xYZ
            match dir {
                ShiftDir::Left => {
                    // a= 0xWY, phl = 0xZX
                    let temp = (a & 0xf0) | (phl >> 4);
                    phl = (phl << 4) | (a & 0x0f);
                    a = temp;
                },
                ShiftDir::Right => {
                    // a= 0xWZ, phl = 0xXY
                    let temp = (a & 0xf0) | (phl & 0x0f);
                    phl = (a << 4) | (phl >> 4);
                    a = temp;
                }
            }
            state.reg.set_a(a);
            state.set_reg(Reg8::_HL, phl);

            state.reg.clear_flag(Flag::H);
            state.reg.clear_flag(Flag::N);
            state.reg.update_sz53p_flags(a);
        })
    }
}
