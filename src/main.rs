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

use cpu::Cpu;
use memory_io::PlainMachine;


fn main() {
    Cpu::new(&mut PlainMachine::new());
}

