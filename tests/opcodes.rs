extern crate z80;

use z80::cpu::Cpu;
use z80::registers::*;
use z80::memory::PlainMemory;



#[test]
fn test_inc_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3c);  // INC A
    cpu.state.reg.set8(REG_A, 0xa4);

    cpu.execute_instruction();

    assert_eq!(0xa5, cpu.state.reg.get8(REG_A));
}

#[test]
fn test_inc_a_overflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3c);  // INC A
    cpu.state.reg.set8(REG_A, 0xff);

    cpu.execute_instruction();

    assert_eq!(0x00, cpu.state.reg.get8(REG_A));
}

#[test]
fn test_inc_e() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x1c);  // INC E
    cpu.state.reg.set8(REG_E, 0x14);

    cpu.execute_instruction();

    assert_eq!(0x15, cpu.state.reg.get8(REG_E));
}

#[test]
fn test_dec_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3d);  // DEC A
    cpu.state.reg.set8(REG_A, 0xa4);

    cpu.execute_instruction();

    assert_eq!(0xa3, cpu.state.reg.get8(REG_A));
}

#[test]
fn test_dec_a_underflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3d);  // DEC A
    cpu.state.reg.set8(REG_A, 0x00);

    cpu.execute_instruction();

    assert_eq!(0xff, cpu.state.reg.get8(REG_A));
}

#[test]
fn test_inc_de() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x13);  // INC DE
    cpu.state.reg.set16(REG_DE, 0xcea4);

    cpu.execute_instruction();

    assert_eq!(0xcea5, cpu.state.reg.get16(REG_DE));
}

#[test]
fn test_inc_de_overflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x13);  // INC DE
    cpu.state.reg.set16(REG_DE, 0xffff);

    cpu.execute_instruction();

    assert_eq!(0x0000, cpu.state.reg.get16(REG_DE));
}

#[test]
fn test_dec_de() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x1b);  // DEC A
    cpu.state.reg.set16(REG_DE, 0x1256);

    cpu.execute_instruction();

    assert_eq!(0x1255, cpu.state.reg.get16(REG_DE));
}

#[test]
fn test_dec_de_underflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x1b);  // DEC DE
    cpu.state.reg.set16(REG_DE, 0x0000);

    cpu.execute_instruction();

    assert_eq!(0xffff, cpu.state.reg.get16(REG_DE));
}

#[test]
fn test_ld_bc_imm() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x01);  // LD BC, $1234
    cpu.state.mem.poke(0x0001, 0x34); 
    cpu.state.mem.poke(0x0002, 0x12); 
    cpu.state.reg.set16(REG_BC, 0x0000);

    cpu.execute_instruction();

    println!("Registers: {:?}", cpu.state.reg);

    assert_eq!(0x1234, cpu.state.reg.get16(REG_BC));
}

#[test]
#[should_panic]
fn test_not_implemented() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xff);

    cpu.execute_instruction();
}
