extern crate z80;

use z80::cpu::Cpu;
use z80::state::State;
use z80::memory_io::*;
use z80::registers::*;

#[test]
fn test_ld_bc_nn() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x01);  // LD BC, $1234
    sys.poke(0x0001, 0x34); 
    sys.poke(0x0002, 0x12); 
    state.reg.set16(Reg16::BC, 0x0000);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x1234, state.reg.get16(Reg16::BC));
}

#[test]
fn test_ld_bc_pnn() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();


    sys.poke(0x0000, 0xed);  // LD BC, ($1234)
    sys.poke(0x0001, 0x4b); 
    sys.poke(0x0002, 0x34); 
    sys.poke(0x0003, 0x12); 
    sys.poke(0x1234, 0x89); 
    sys.poke(0x1235, 0x67); 
    state.reg.set16(Reg16::BC, 0x0000);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x6789, state.reg.get16(Reg16::BC));
}

#[test]
fn test_ld_pnn_bc() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();


    sys.poke(0x0000, 0xed);  // LD ($1234), BC
    sys.poke(0x0001, 0x43); 
    sys.poke(0x0002, 0x34); 
    sys.poke(0x0003, 0x12); 
    state.reg.set16(Reg16::BC, 0xde23);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xde23, sys.peek16(0x1234));
}

#[test]
fn test_ld_a_b() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x78);  // LD A, B
    state.reg.set8(Reg8::B, 0x23);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x23, state.reg.get_a());
}

#[test]
fn test_ld_b_n() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x06);  // LD B, $34
    sys.poke(0x0001, 0x34); 
    state.reg.set8(Reg8::B, 0x9e);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0x34, state.reg.get8(Reg8::B));
}

#[test]
fn test_ld_d_e() {
    let mut sys = PlainMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    sys.poke(0x0000, 0x53);  // LD D, E
    sys.poke(0x0001, 0x34); 
    state.reg.set8(Reg8::D, 0xdd);
    state.reg.set8(Reg8::E, 0xee);

    cpu.execute_instruction(&mut state, &mut sys);

    assert_eq!(0xee, state.reg.get8(Reg8::D));
    assert_eq!(0xee, state.reg.get8(Reg8::E));
}
