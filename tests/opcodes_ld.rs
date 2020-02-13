extern crate z80;

use z80::cpu::Cpu;
use z80::registers::*;

#[test]
fn test_ld_bc_nn() {
    let mut cpu = Cpu::new_plain();
    cpu.state.mem.poke(0x0000, 0x01);  // LD BC, $1234
    cpu.state.mem.poke(0x0001, 0x34); 
    cpu.state.mem.poke(0x0002, 0x12); 
    cpu.state.reg.set16(Reg16::BC, 0x0000);

    cpu.execute_instruction();

    assert_eq!(0x1234, cpu.state.reg.get16(Reg16::BC));
}

#[test]
fn test_ld_bc_pnn() {
    let mut cpu = Cpu::new_plain();

    cpu.state.mem.poke(0x0000, 0xed);  // LD BC, ($1234)
    cpu.state.mem.poke(0x0001, 0x4b); 
    cpu.state.mem.poke(0x0002, 0x34); 
    cpu.state.mem.poke(0x0003, 0x12); 
    cpu.state.mem.poke(0x1234, 0x89); 
    cpu.state.mem.poke(0x1235, 0x67); 
    cpu.state.reg.set16(Reg16::BC, 0x0000);

    cpu.execute_instruction();

    assert_eq!(0x6789, cpu.state.reg.get16(Reg16::BC));
}

#[test]
fn test_ld_pnn_bc() {
    let mut cpu = Cpu::new_plain();

    cpu.state.mem.poke(0x0000, 0xed);  // LD ($1234), BC
    cpu.state.mem.poke(0x0001, 0x43); 
    cpu.state.mem.poke(0x0002, 0x34); 
    cpu.state.mem.poke(0x0003, 0x12); 
    cpu.state.reg.set16(Reg16::BC, 0xde23);

    cpu.execute_instruction();

    assert_eq!(0xde23, cpu.state.mem.peek16(0x1234));
}

#[test]
fn test_ld_a_b() {
    let mut cpu = Cpu::new_plain();
    cpu.state.mem.poke(0x0000, 0x78);  // LD A, B
    cpu.state.reg.set8(Reg8::B, 0x23);

    cpu.execute_instruction();

    assert_eq!(0x23, cpu.state.reg.get8(Reg8::A));
}

#[test]
fn test_ld_b_n() {
    let mut cpu = Cpu::new_plain();
    cpu.state.mem.poke(0x0000, 0x06);  // LD B, $34
    cpu.state.mem.poke(0x0001, 0x34); 
    cpu.state.reg.set8(Reg8::B, 0x9e);

    cpu.execute_instruction();

    assert_eq!(0x34, cpu.state.reg.get8(Reg8::B));
}

#[test]
fn test_ld_d_e() {
    let mut cpu = Cpu::new_plain();
    cpu.state.mem.poke(0x0000, 0x53);  // LD D, E
    cpu.state.mem.poke(0x0001, 0x34); 
    cpu.state.reg.set8(Reg8::D, 0xdd);
    cpu.state.reg.set8(Reg8::E, 0xee);

    cpu.execute_instruction();

    assert_eq!(0xee, cpu.state.reg.get8(Reg8::D));
    assert_eq!(0xee, cpu.state.reg.get8(Reg8::E));
}
