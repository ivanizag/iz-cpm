#![allow(dead_code)]

mod decoder;
mod memory_io;
mod opcode;
mod opcode_arith;
mod opcode_bits;
mod opcode_io;
mod opcode_jumps;
mod opcode_ld;
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
use zexio::ZexIo;

fn mainold() {
    Cpu::new_plain();
}

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
    cpu.state.mem.poke(5, 0xdb); // IN A, $5
    cpu.state.mem.poke(6, 0x05);
    cpu.state.mem.poke(7, 0xc9); // RET


    cpu.state.reg.set_pc(0x100);
    loop {
        cpu.execute_instruction();
    }

}



