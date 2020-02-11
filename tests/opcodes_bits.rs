extern crate z80;

use z80::cpu::Cpu;
use z80::registers::*;
use z80::memory::PlainMemory;

#[test]
fn test_rrca_fast() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x0f); // RRCA
    cpu.state.reg.set8(Reg8::A, 0b10010011);
    cpu.state.reg.set_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b11001001, cpu.state.reg.get8(Reg8::A));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_rrc_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // RRC A
    cpu.state.mem.poke(0x0001, 0x0f);
    cpu.state.reg.set8(Reg8::A, 0b10010011);
    cpu.state.reg.set_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b11001001, cpu.state.reg.get8(Reg8::A));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_rr_b() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // RR B
    cpu.state.mem.poke(0x0001, 0x18);
    cpu.state.reg.set8(Reg8::B, 0b10010010);
    cpu.state.reg.set_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b11001001, cpu.state.reg.get8(Reg8::B));
    assert_eq!(false, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_sra_c() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // SRA C
    cpu.state.mem.poke(0x0001, 0x29);
    cpu.state.reg.set8(Reg8::C, 0b10010011);
    cpu.state.reg.clear_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b11001001, cpu.state.reg.get8(Reg8::C));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_srl_d() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // SRL D
    cpu.state.mem.poke(0x0001, 0x3a);
    cpu.state.reg.set8(Reg8::D, 0b10010011);
    cpu.state.reg.clear_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b01001001, cpu.state.reg.get8(Reg8::D));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_rlc_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // RLC A
    cpu.state.mem.poke(0x0001, 0x07);
    cpu.state.reg.set8(Reg8::A, 0b00010011);
    cpu.state.reg.set_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b00100110, cpu.state.reg.get8(Reg8::A));
    assert_eq!(false, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_rl_b() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // RL B
    cpu.state.mem.poke(0x0001, 0x10);
    cpu.state.reg.set8(Reg8::B, 0b00010011);
    cpu.state.reg.set_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b00100111, cpu.state.reg.get8(Reg8::B));
    assert_eq!(false, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_sla_c() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // SLA C
    cpu.state.mem.poke(0x0001, 0x21);
    cpu.state.reg.set8(Reg8::C, 0b10010011);
    cpu.state.reg.clear_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b00100110, cpu.state.reg.get8(Reg8::C));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_sll_d() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // SLL D
    cpu.state.mem.poke(0x0001, 0x32);
    cpu.state.reg.set8(Reg8::D, 0b10010011);
    cpu.state.reg.clear_flag(Flag::C);

    cpu.execute_instruction();

    assert_eq!(0b00100111, cpu.state.reg.get8(Reg8::D));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::C));
}

#[test]
fn test_bit_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // BIT 1, A
    cpu.state.mem.poke(0x0001, 0x4f);
    cpu.state.reg.set8(Reg8::A, 0b00010010);

    cpu.execute_instruction();

    assert_eq!(0b00010010, cpu.state.reg.get8(Reg8::A));
    assert_eq!(true, cpu.state.reg.get_flag(Flag::Z));
}

#[test]
fn test_set_b() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // SET 0, B
    cpu.state.mem.poke(0x0001, 0xc0);
    cpu.state.reg.set8(Reg8::B, 0b00010010);

    cpu.execute_instruction();

    assert_eq!(0b00010011, cpu.state.reg.get8(Reg8::B));
    assert_eq!(false, cpu.state.reg.get_flag(Flag::Z));
}

#[test]
fn test_res_c() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcb); // RES 7, C
    cpu.state.mem.poke(0x0001, 0xb9);
    cpu.state.reg.set8(Reg8::C, 0b10010011);

    cpu.execute_instruction();

    assert_eq!(0b00010011, cpu.state.reg.get8(Reg8::C));
    assert_eq!(false, cpu.state.reg.get_flag(Flag::Z));
}

#[test]
fn test_cpl() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x2f);  // CPL
    cpu.state.reg.set8(Reg8::A, 0x3d);

    cpu.execute_instruction();

    assert_eq!(0xc2, cpu.state.reg.get8(Reg8::A));
}