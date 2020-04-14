extern crate z80;

use z80::cpu::Cpu;
use z80::state::State;
use z80::memory_io::*;
use z80::registers::*;

#[test]
fn test_rrca_fast() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x0f); // RRCA
    state.reg.set_a(0b10010011);
    state.reg.set_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b11001001, state.reg.get_a());
    assert_eq!(true, state.reg.get_flag(Flag::C));
}

#[test]
fn test_rrc_a() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RRC A
    sys.poke(0x0001, 0x0f);
    state.reg.set_a(0b10010011);
    state.reg.set_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b11001001, state.reg.get_a());
    assert_eq!(true, state.reg.get_flag(Flag::C));
}

#[test]
fn test_rr_b() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RR B
    sys.poke(0x0001, 0x18);
    state.reg.set8(Reg8::B, 0b10010010);
    state.reg.set_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b11001001, state.reg.get8(Reg8::B));
    assert_eq!(false, state.reg.get_flag(Flag::C));
}

#[test]
fn test_sra_c() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SRA C
    sys.poke(0x0001, 0x29);
    state.reg.set8(Reg8::C, 0b10010011);
    state.reg.clear_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b11001001, state.reg.get8(Reg8::C));
    assert_eq!(true, state.reg.get_flag(Flag::C));
}

#[test]
fn test_srl_d() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SRL D
    sys.poke(0x0001, 0x3a);
    state.reg.set8(Reg8::D, 0b10010011);
    state.reg.clear_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b01001001, state.reg.get8(Reg8::D));
    assert_eq!(true, state.reg.get_flag(Flag::C));
}

#[test]
fn test_rlc_a() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RLC A
    sys.poke(0x0001, 0x07);
    state.reg.set_a(0b00010011);
    state.reg.set_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00100110, state.reg.get_a());
    assert_eq!(false, state.reg.get_flag(Flag::C));
}

#[test]
fn test_rl_b() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RL B
    sys.poke(0x0001, 0x10);
    state.reg.set8(Reg8::B, 0b00010011);
    state.reg.set_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00100111, state.reg.get8(Reg8::B));
    assert_eq!(false, state.reg.get_flag(Flag::C));
}

#[test]
fn test_sla_c() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SLA C
    sys.poke(0x0001, 0x21);
    state.reg.set8(Reg8::C, 0b10010011);
    state.reg.clear_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00100110, state.reg.get8(Reg8::C));
    assert_eq!(true, state.reg.get_flag(Flag::C));
}

#[test]
fn test_sll_d() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SLL D
    sys.poke(0x0001, 0x32);
    state.reg.set8(Reg8::D, 0b10010011);
    state.reg.clear_flag(Flag::C);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00100111, state.reg.get8(Reg8::D));
    assert_eq!(true, state.reg.get_flag(Flag::C));
}

#[test]
fn test_bit_a() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // BIT 1, A
    sys.poke(0x0001, 0x4f);
    state.reg.set_a(0b00010010);
    state.reg.set_flag(Flag::Z);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00010010, state.reg.get_a());
    assert_eq!(false, state.reg.get_flag(Flag::Z));
}

#[test]
fn test_set_b() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // SET 0, B
    sys.poke(0x0001, 0xc0);
    state.reg.set8(Reg8::B, 0b00010010);
    state.reg.clear_flag(Flag::Z);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00010011, state.reg.get8(Reg8::B));
    assert_eq!(false, state.reg.get_flag(Flag::Z));
}

#[test]
fn test_res_c() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xcb); // RES 7, C
    sys.poke(0x0001, 0xb9);
    state.reg.set8(Reg8::C, 0b10010011);
    state.reg.clear_flag(Flag::Z);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0b00010011, state.reg.get8(Reg8::C));
    assert_eq!(false, state.reg.get_flag(Flag::Z));
}

#[test]
fn test_cpl() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x2f);  // CPL
    state.reg.set_a(0x3d);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xc2, state.reg.get_a());
}

#[test]
fn test_rld() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed); // RLD
    sys.poke(0x0001, 0x6f);
    state.reg.set_a(0xab);
    state.reg.set16(Reg16::HL, 0xccdd);
    sys.poke(0xccdd, 0xcd);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xac, state.reg.get_a());
    assert_eq!(0xdb, sys.peek(0xccdd));
}

#[test]
fn test_rrd() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0xed); // RRD
    sys.poke(0x0001, 0x67);
    state.reg.set_a(0xab);
    state.reg.set16(Reg16::HL, 0xccdd);
    sys.poke(0xccdd, 0xcd);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xad, state.reg.get_a());
    assert_eq!(0xbc, sys.peek(0xccdd));
}