#![allow(dead_code)]

mod decoder;
mod memory_io;
mod opcode;
mod opcode_alu;
mod opcode_arith;
mod opcode_bits;
mod opcode_io;
mod opcode_jumps;
mod opcode_ld;
mod operators;
mod registers;
mod state;
mod cpu;
mod zexio;

use std::fs::File;
use std::io::prelude::*;
use cpu::Cpu;
use state::State;
use memory_io::PlainMemoryIo;
use decoder::Decoder;
use registers::Reg16;
use registers::Reg8;
use zexio::ZexIo;

fn mainold() {
    Cpu::new_plain();
}

/*
 Profile with:
RUSTFLAGS='-g' perf record --call-graph=dwarf cargo run --release
perf report

*/

fn main() {
    let mut cpu = Cpu {
        state: State::new(
            Box::new(PlainMemoryIo::new()),
            Box::new(ZexIo{})),
        decoder: Decoder::new()
    };

    // Load program
    let mut file = File::open("tests/zexdoc.com").unwrap();
    let mut buf = [0u8;65536];
    let size = file.read(&mut buf).unwrap();
    for i in 0..size {
        cpu.state.mem.poke(0x100 + i as u16, buf[i]);
    }

    // Prepare system calls
    cpu.state.mem.poke(5, 0xdb); // IN A, 0
    cpu.state.mem.poke(6, 0x00);
    cpu.state.mem.poke(7, 0xc9); // RET

    //println!("Testing \"testfiles/zexdoc.com\"...");
    cpu.state.reg.set_pc(0x100);
    let trace = false;
    loop {
        cpu.execute_instruction();

        if trace {
            // CPU regs
            println!("PC({:04x}) AF({:04x}) BC({:04x}) DE({:04x}) HL({:04x}) SP({:04x}) IX({:04x}) IY({:04x}) Flags({:08b})",
                cpu.state.reg.get_pc(),
                cpu.state.reg.get16(Reg16::AF),
                cpu.state.reg.get16(Reg16::BC),
                cpu.state.reg.get16(Reg16::DE),
                cpu.state.reg.get16(Reg16::HL),
                cpu.state.reg.get16(Reg16::SP),
                cpu.state.reg.get16(Reg16::IX),
                cpu.state.reg.get16(Reg16::IY),
                cpu.state.reg.get8(Reg8::F)
            );

            // Test state
            let addr = 0x1d80 as u16;
            print!("Zex state 0x{:04x}: ", addr);
            for i in 0..0x10 {
                print!("{:02x} ", cpu.state.mem.peek(addr + i));
            }
            println!("");
        }

        if cpu.state.peek16_pc() == 0xfff0 {
            return;
        }
        //println!("Z: {}", cpu.state.reg.get_flag(Flag::C));
    }
}



