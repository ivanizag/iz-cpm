extern crate z80;

use z80::cpu::Cpu;
use z80::registers::*;

#[test]
fn test_two_opcodes() {
    let mut cpu = Cpu::new_plain();
    cpu.state.sys.poke(0x0000, 0x06);  // LD B, $34
    cpu.state.sys.poke(0x0001, 0x34);
    cpu.state.sys.poke(0x0002, 0x78);  // LD A, B
 
    cpu.execute_instruction();
    cpu.execute_instruction();

    println!("Registers: {:?}", cpu.state.reg);

    assert_eq!(0x34, cpu.state.reg.get_a());
}

#[test]
fn test_push_pop_rr() {
    let mut cpu = Cpu::new_plain();
    cpu.state.sys.poke(0x0000, 0xc5);  // PUSH BC
    cpu.state.sys.poke(0x0001, 0xf1);  // POP AF
    cpu.state.reg.set16(Reg16::AF, 0x5678);
    cpu.state.reg.set16(Reg16::BC, 0x1234);

    cpu.execute_instruction();
    assert_eq!(0x1234, cpu.state.reg.get16(Reg16::BC));
    assert_eq!(0x5678, cpu.state.reg.get16(Reg16::AF));

    cpu.execute_instruction();
    assert_eq!(0x1234, cpu.state.reg.get16(Reg16::BC));
    assert_eq!(0x1234, cpu.state.reg.get16(Reg16::AF));
}
