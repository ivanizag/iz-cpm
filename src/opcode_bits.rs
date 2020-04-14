use super::opcode::*;
use super::environment::*;
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

pub fn build_rot_r(r: Reg8, (dir, mode, name): (ShiftDir, ShiftMode, &str), fast: bool, indexed: bool) -> Opcode {
    let full_name: String;
    if indexed {
        full_name = format!("LD {}, {} {}", r, name, Reg8::_HL);
    } else {
        let separator = if fast {""} else {" "};
        full_name = format!("{}{}{}", name, separator, r);
    }
    Opcode {
        name: full_name,
        cycles: if fast {4} else {8}, // The one byte opcodes are faster // (HL): 15, (IX+d): 23
        action: Box::new(move |env: &mut Environment| {
            env.load_displacement(r);

            let mut v = if indexed {
                env.get_reg(Reg8::_HL)
            } else {
                env.get_reg(r)
            };

            let carry: bool;
            match dir {
                ShiftDir::Left => {
                    let upper_bit = v >= 0x80;
                    v = v << 1;
                    let set_lower_bit = match mode {
                        ShiftMode::Arithmetic => false, // always 0 in bit 0
                        ShiftMode::Logical => true, // always 1 in bit 0
                        ShiftMode::Rotate => env.state.reg.get_flag(Flag::C), // carry in bit 0
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
                        ShiftMode::Rotate => env.state.reg.get_flag(Flag::C), // carry in bit 0
                        ShiftMode::RotateCarry => lower_bit, // bit 0 goes to bit 7
                    };
                    if set_upper_bit { // bit 7 is 0 already
                        v = v | 0x80;
                    }
                    carry = lower_bit;
                }
            }
            if indexed && r != Reg8::_HL {
                env.set_reg(Reg8::_HL, v);
            }
            env.set_reg(r, v);

            env.state.reg.put_flag(Flag::C, carry);
            env.state.reg.clear_flag(Flag::H);
            env.state.reg.clear_flag(Flag::N);
            if fast {
                env.state.reg.update_53_flags(v);
            } else {
                env.state.reg.update_sz53p_flags(v);
            }
        })
    }
}

pub fn build_bit_r(n: u8, r: Reg8) -> Opcode {
    Opcode {
        name: format!("BIT {}, {}", n, r),
        cycles: 8, // (HL) 12, (IX+d) 20
        action: Box::new(move |env: &mut Environment| {
            env.load_displacement(r);

            let v = env.get_reg(r);
            let z = v & (1<<n);
            env.state.reg.put_flag(Flag::S, (z & 0x80) != 0);
            env.state.reg.update_53_flags(v); // TUZD-4.1, copy bits from reg
            env.state.reg.put_flag(Flag::Z, z == 0);
            env.state.reg.set_flag(Flag::H);
            env.state.reg.put_flag(Flag::P, z == 0);
            env.state.reg.clear_flag(Flag::N);

            if r == Reg8::_HL {
                // Exceptions for (IX+d) TUZD-4-1
                /* With the BIT n,(IX+d) instructions, the flags behave just
                like the BIT n,r instruction, except for YF and XF. These are
                not copied from the result but from something completely
                different, namely bit 5 and 3 of the high byte of IX+d (so IX
                plus the displacement).
                */
                let address = env.get_index_address();
                env.state.reg.update_53_flags((address >> 8) as u8);

                // Exceptions for (HL) TUZD-4-1
                /* Things get more bizarre with the BIT n,(HL)
                instruction. Again, except for YF and XF the flags
                are the same. YF and XF are copied from some sort
                of internal register */
                // Not implemented. Just done the same than for (IX+d)
            }
        })
    }
}

pub fn build_set_res_r(bit: u8, r: Reg8, value: bool) -> Opcode {
    let name = if value {"SET"} else {"RES"};
    Opcode {
        name: format!("{} {}, {}", name, bit, r),
        cycles: 8, // (HL) 15
        action: Box::new(move |env: &mut Environment| {
            env.load_displacement(r);

            let mut v = env.get_reg(r);
            if value {
                v = v | (1<<bit);
            } else {
                v = v & !(1<<bit);
            }

            env.set_reg(r, v);
        })
    }
}

pub fn build_indexed_set_res_r(bit: u8, r: Reg8, value: bool) -> Opcode {
    let name = if value {"SET"} else {"RES"};
    Opcode {
        name: format!("LD {}, {} {}, {}", r, name, bit, Reg8::_HL),
        cycles: 23,
        action: Box::new(move |env: &mut Environment| {
            /*
            An instruction such as LD r, RES b, (IX+d) should be interpreted as
            "attempt to reset bit b of the byte at (IX+d), and copy the result
            to register r, even the new byte cannot be written at the said
            address (e.g. when it points to a ROM location).
            */
            env.load_displacement(r);

            let mut v = env.get_reg(Reg8::_HL);
            if value {
                v = v | (1<<bit);
            } else {
                v = v & !(1<<bit);
            }
            env.set_reg(Reg8::_HL, v);
            if r != Reg8::_HL {
                env.set_reg(r, v);
            }
        })
    }
}



pub fn build_cpl() -> Opcode {
    Opcode {
        name: "CPL".to_string(),
        cycles: 4,
        action: Box::new(move |env: &mut Environment| {
            let mut v = env.state.reg.get_a();
            v = !v;
            env.state.reg.set_a(v);

            env.state.reg.set_flag(Flag::H);
            env.state.reg.set_flag(Flag::N);
            env.state.reg.update_53_flags(v);
        })
    }
}

pub fn build_scf() -> Opcode {
    Opcode {
        name: "SCF".to_string(),
        cycles: 4,
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.get_a();

            env.state.reg.set_flag(Flag::C);
            env.state.reg.clear_flag(Flag::H);
            env.state.reg.clear_flag(Flag::N);
            env.state.reg.update_53_flags(a);
        })
    }
}

pub fn build_ccf() -> Opcode {
    Opcode {
        name: "CCF".to_string(),
        cycles: 4,
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.get_a();
            let c = env.state.reg.get_flag(Flag::C);

            env.state.reg.put_flag(Flag::C, !c);
            env.state.reg.put_flag(Flag::H, c);
            env.state.reg.clear_flag(Flag::N);
            env.state.reg.update_53_flags(a);
        })
    }
}

pub fn build_rxd(dir: ShiftDir, name: &str) -> Opcode {
    Opcode {
        name: name.to_string(),
        cycles: 18,
        action: Box::new(move |env: &mut Environment| {
            let mut a = env.state.reg.get_a();
            let mut phl = env.get_reg(Reg8::_HL);
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
            env.state.reg.set_a(a);
            env.set_reg(Reg8::_HL, phl);

            env.state.reg.clear_flag(Flag::H);
            env.state.reg.clear_flag(Flag::N);
            env.state.reg.update_sz53p_flags(a);
        })
    }
}
