#![allow(dead_code)]

mod decoder;
mod memory;
mod opcode;
mod opcode_jumps;
mod opcode_ld;
mod registers;
mod state;
mod cpu;

use cpu::Cpu;
use memory::PlainMemory;

fn main() {
    Cpu::new(Box::new(PlainMemory::new()));
}
