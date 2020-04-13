extern crate z80;

use z80::cpu::Cpu;
use z80::memory_io::PlainMachine;
use z80::registers::*;

#[test]
fn test_out_e() {
    let mut machine = PlainMachine::new();
    let mut cpu = Cpu::new(&mut machine);
    cpu.state.sys.poke(0x0000, 0xed); // OUT (C), E
    cpu.state.sys.poke(0x0001, 0x59);
    cpu.state.reg.set8(Reg8::E, 0x63);
    cpu.state.reg.set16(Reg16::BC, 0x6345);

    cpu.execute_instruction();

    assert_eq!(0x63, cpu.state.port_in(0x6345));
}

#[test]
fn test_in_e() {
    let mut machine = PlainMachine::new();
    let mut cpu = Cpu::new(&mut machine);
    cpu.state.sys.poke(0x0000, 0xed); // IN E, (C)
    cpu.state.sys.poke(0x0001, 0x58);
    cpu.state.reg.set16(Reg16::BC, 0x6345);
    cpu.state.port_out(0x6345, 0x8a);

    cpu.execute_instruction();

    assert_eq!(0x8a, cpu.state.port_in(0x6345));
}
