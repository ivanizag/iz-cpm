extern crate z80;

use z80::cpu::Cpu;
use z80::registers::*;
use z80::memory::PlainMemory;



#[test]
#[should_panic]
fn test_not_implemented() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xff);

    cpu.execute_instruction();
}

#[test]
fn test_two_opcodes() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x06);  // LD B, $34
    cpu.state.mem.poke(0x0001, 0x34); 
    cpu.state.mem.poke(0x0002, 0x78);  // LD A, B
 
    cpu.execute_instruction();
    cpu.execute_instruction();

    println!("Registers: {:?}", cpu.state.reg);

    assert_eq!(0x34, cpu.state.reg.get8(&Register8::A));
}

#[test]
fn test_inc_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3c);  // INC A
    cpu.state.reg.set8(&Register8::A, 0xa4);

    cpu.execute_instruction();

    assert_eq!(0xa5, cpu.state.reg.get8(&Register8::A));
}

#[test]
fn test_inc_a_overflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3c);  // INC A
    cpu.state.reg.set8(&Register8::A, 0xff);

    cpu.execute_instruction();

    assert_eq!(0x00, cpu.state.reg.get8(&Register8::A));
}

#[test]
fn test_inc_e() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x1c);  // INC E
    cpu.state.reg.set8(&Register8::E, 0x14);

    cpu.execute_instruction();

    assert_eq!(0x15, cpu.state.reg.get8(&Register8::E));
}

#[test]
fn test_dec_a() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3d);  // DEC A
    cpu.state.reg.set8(&Register8::A, 0xa4);

    cpu.execute_instruction();

    assert_eq!(0xa3, cpu.state.reg.get8(&Register8::A));
}

#[test]
fn test_dec_a_underflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x3d);  // DEC A
    cpu.state.reg.set8(&Register8::A, 0x00);

    cpu.execute_instruction();

    assert_eq!(0xff, cpu.state.reg.get8(&Register8::A));
}

#[test]
fn test_inc_de() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x13);  // INC DE
    cpu.state.reg.set16(&Register16::DE, 0xcea4);

    cpu.execute_instruction();

    assert_eq!(0xcea5, cpu.state.reg.get16(&Register16::DE));
}

#[test]
fn test_inc_de_overflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x13);  // INC DE
    cpu.state.reg.set16(&Register16::DE, 0xffff);

    cpu.execute_instruction();

    assert_eq!(0x0000, cpu.state.reg.get16(&Register16::DE));
}

#[test]
fn test_dec_de() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x1b);  // DEC A
    cpu.state.reg.set16(&Register16::DE, 0x1256);

    cpu.execute_instruction();

    assert_eq!(0x1255, cpu.state.reg.get16(&Register16::DE));
}

#[test]
fn test_dec_de_underflow() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x1b);  // DEC DE
    cpu.state.reg.set16(&Register16::DE, 0x0000);

    cpu.execute_instruction();

    assert_eq!(0xffff, cpu.state.reg.get16(&Register16::DE));
}

#[test]
fn test_add_hl_de() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x19);  // ADD HL, DE
    cpu.state.reg.set16(&Register16::HL, 0x1234);
    cpu.state.reg.set16(&Register16::DE, 0x0101);

    cpu.execute_instruction();

    assert_eq!(0x1335, cpu.state.reg.get16(&Register16::HL));
}
