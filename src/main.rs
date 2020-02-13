#![allow(dead_code)]

mod decoder;
mod memory;
mod opcode;
mod opcode_arith;
mod opcode_bits;
mod opcode_io;
mod opcode_jumps;
mod opcode_ld;
mod registers;
mod state;
mod cpu;

use cpu::Cpu;

fn main() {
    Cpu::new_plain();
}
