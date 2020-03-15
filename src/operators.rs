use super::state::*;
use super::registers::*;

pub type Operator = fn(&mut State, u8, u8) -> u8;

pub fn operator_add(_state: &mut State, _a: u8, _b: u8) -> u8 {
    panic!("Not implemented");
}

pub fn operator_adc(_state: &mut State, _a: u8, _b: u8) -> u8 {
    panic!("Not implemented");
}

pub fn operator_sub(_state: &mut State, _a: u8, _b: u8) -> u8 {
    panic!("Not implemented");
}

pub fn operator_sbc(_state: &mut State, _a: u8, _b: u8) -> u8 {
    panic!("Not implemented");
}

pub fn operator_and(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a & b;

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::N);
    state.reg.set_flag(Flag::H);
    v
}

pub fn operator_xor(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a ^ b;

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::N);
    state.reg.clear_flag(Flag::H);
    v
}

pub fn operator_or(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a | b;

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::N);
    state.reg.clear_flag(Flag::H);
    v
}

pub fn operator_cp(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a.wrapping_sub(b);

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.set_flag(Flag::N);

    //TODO: flags
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::H);
    a // Do not update the accumulator
}
