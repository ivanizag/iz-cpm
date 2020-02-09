extern crate z80;

use z80::cpu::Cpu;
use z80::registers::*;
use z80::memory::PlainMemory;

#[test]
fn test_djnz_jump() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x10);  // DJNZ +$06
    cpu.state.mem.poke(0x0001, 0x06); 
    cpu.state.reg.set8(Reg8::B, 0x23);

    cpu.execute_instruction();
    assert_eq!(0x22, cpu.state.reg.get8(Reg8::B));
    assert_eq!(0x0006, cpu.state.reg.get_pc());
}

#[test]
fn test_djnz_no_jump() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x10);  // DJNZ +$06
    cpu.state.mem.poke(0x0001, 0x06); 
    cpu.state.reg.set8(Reg8::B, 0x01);

    cpu.execute_instruction();
    assert_eq!(0x00, cpu.state.reg.get8(Reg8::B));
    assert_eq!(0x0002, cpu.state.reg.get_pc());
}

#[test]
fn test_jr_z_jump() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0x10);  // JR -$02
    cpu.state.mem.poke(0x0001, 0xfe); 
    cpu.state.reg.set_flag(Flag::Z);

    cpu.execute_instruction();
    assert_eq!(0xFFFE, cpu.state.reg.get_pc());
}

#[test]
fn test_jp() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xc3);  // JP $2000
    cpu.state.mem.poke(0x0001, 0x00); 
    cpu.state.mem.poke(0x0002, 0x20);
    
    cpu.execute_instruction();
    assert_eq!(0x2000, cpu.state.reg.get_pc());
}

#[test]
fn test_call() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcd);  // CALL $2000
    cpu.state.mem.poke(0x0001, 0x00); 
    cpu.state.mem.poke(0x0002, 0x20);
    
 
    cpu.execute_instruction();
    assert_eq!(0x2000, cpu.state.reg.get_pc());
    assert_eq!(0x0003, cpu.state.pop16());
}

#[test]
fn test_call_z_jump() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcc);  // CALL Z $2000
    cpu.state.mem.poke(0x0001, 0x00); 
    cpu.state.mem.poke(0x0002, 0x20);
    cpu.state.reg.set_flag(Flag::Z);
     
    cpu.execute_instruction();
    assert_eq!(0x2000, cpu.state.reg.get_pc());
    assert_eq!(0x0003, cpu.state.pop16());
}

#[test]
fn test_call_z_no_jump() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcc);  // CALL Z $2000
    cpu.state.mem.poke(0x0001, 0x00); 
    cpu.state.mem.poke(0x0002, 0x20);
    cpu.state.reg.clear_flag(Flag::Z);
     
    cpu.execute_instruction();
    assert_eq!(0x0003, cpu.state.reg.get_pc());
}


#[test]
fn test_call_ret() {
    let mut cpu = Cpu::new(Box::new(PlainMemory::new()));
    cpu.state.mem.poke(0x0000, 0xcd);  // CALL $2000
    cpu.state.mem.poke(0x0001, 0x00); 
    cpu.state.mem.poke(0x0002, 0x20);

    cpu.state.mem.poke(0x2000, 0xc9);  // RET
    
    cpu.execute_instruction();
    assert_eq!(0x2000, cpu.state.reg.get_pc());
     cpu.execute_instruction();
    assert_eq!(0x0003, cpu.state.reg.get_pc());
}
