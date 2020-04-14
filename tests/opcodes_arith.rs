extern crate z80;

use z80::cpu::Cpu;
use z80::state::State;
use z80::memory_io::*;
use z80::registers::*;

#[test]
fn test_neg_a() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed);  // NEG
    sys.poke(0x0001, 0x44);
    state.reg.set_a(0xff);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x01, state.reg.get_a());
}

#[test]
fn test_inc_a() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3c);  // INC A
    state.reg.set_a(0xa4);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xa5, state.reg.get_a());
}

#[test]
fn test_inc_a_overflow() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3c);  // INC A
    state.reg.set_a(0xff);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x00, state.reg.get_a());
}

#[test]
fn test_inc_e() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x1c);  // INC E
    state.reg.set8(Reg8::E, 0x14);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x15, state.reg.get8(Reg8::E));
}

#[test]
fn test_dec_a() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3d);  // DEC A
    state.reg.set_a(0xa4);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xa3, state.reg.get_a());
}

#[test]
fn test_dec_a_underflow() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x3d);  // DEC A
    state.reg.set_a(0x00);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xff, state.reg.get_a());
}

#[test]
fn test_inc_de() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x13);  // INC DE
    state.reg.set16(Reg16::DE, 0xcea4);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xcea5, state.reg.get16(Reg16::DE));
}

#[test]
fn test_inc_de_overflow() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x13);  // INC DE
    state.reg.set16(Reg16::DE, 0xffff);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x0000, state.reg.get16(Reg16::DE));
}

#[test]
fn test_dec_de() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x1b);  // DEC A
    state.reg.set16(Reg16::DE, 0x1256);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x1255, state.reg.get16(Reg16::DE));
}

#[test]
fn test_dec_de_underflow() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x1b);  // DEC DE
    state.reg.set16(Reg16::DE, 0x0000);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xffff, state.reg.get16(Reg16::DE));
}

#[test]
fn test_dec_phl() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x35);  // DEC (HL)
    state.reg.set16(Reg16::HL, 0x23c4);
    sys.poke(0x23c4, 0x67);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x66, sys.peek(0x23c4));
}

#[test]
fn test_add_hl_de() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x19);  // ADD HL, DE
    state.reg.set16(Reg16::HL, 0x1234);
    state.reg.set16(Reg16::DE, 0x0101);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x1335, state.reg.get16(Reg16::HL));
}
