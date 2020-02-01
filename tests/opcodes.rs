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