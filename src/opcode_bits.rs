use super::opcode::*;
use super::state::*;
use super::registers::*;

// Relative jumps

#[derive(Copy, Clone)]
pub enum ShiftMode {
    Arithmetic,
    Logical,
    Rotate,
    RotateCarry
}

pub fn build_left_r(r: Reg8, mode: ShiftMode, fast: bool) -> Opcode {
    let separator = if fast {""} else {" "};
    let mnemonic = match mode {
        ShiftMode::Arithmetic => "SLA",
        ShiftMode::Logical => "SLL", // Undocumented
        ShiftMode::Rotate => "RL",
        ShiftMode::RotateCarry => "RLC"
    };

    Opcode {
        name: format!("{}{}{:?}", mnemonic, separator, r),
        bytes: 1,
        cycles: if fast {4} else {8},
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(r);
            let upper_bit = v >= 0x80;
            v = v << 1;
            let set_lower_bit = match mode {
                ShiftMode::Arithmetic => false, // always 0 in bit 0
                ShiftMode::Logical => true, // always 1 in bit 0
                ShiftMode::Rotate => state.reg.get_flag(Flag::C), // carry in bit 0
                ShiftMode::RotateCarry => upper_bit, // bit 7 moves to bit 0
            };
            println!("left {} {} {}", v, upper_bit, set_lower_bit);
            if set_lower_bit { // bit 0 is 0 already
                v = v | 1;
            }

            println!("left {} {} {}", v, upper_bit, set_lower_bit);

            state.reg.set8(r, v);
            state.reg.put_flag(Flag::C, upper_bit);

            state.reg.clear_flag(Flag::H);
            state.reg.clear_flag(Flag::N);
            if !fast {
                state.reg.update_sz53_flags(v);
                state.reg.update_p_flag(v);
            }
        })
    }
}

pub fn build_right_r(r: Reg8, mode: ShiftMode, fast: bool) -> Opcode {
    let separator = if fast {""} else {" "};
    let mnemonic = match mode {
        ShiftMode::Arithmetic => "SRA",
        ShiftMode::Logical => "SRL",
        ShiftMode::Rotate => "RR",
        ShiftMode::RotateCarry => "RRC",
    };

    Opcode {
        name: format!("{}{}{:?}", mnemonic, separator, r),
        bytes: 1,
        cycles: if fast {4} else {8},
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(r);
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

            println!("{} {} {} {}", v, upper_bit, lower_bit, set_upper_bit);

            state.reg.set8(r, v);
            state.reg.put_flag(Flag::C, lower_bit);

            state.reg.clear_flag(Flag::H);
            state.reg.clear_flag(Flag::N);
            if !fast {
                state.reg.update_sz53_flags(v);
                state.reg.update_p_flag(v);
            }
        })
    }
}
