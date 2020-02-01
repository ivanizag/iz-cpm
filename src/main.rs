#![allow(dead_code)]

mod decoder;
mod memory;
mod registers;
mod state;


fn main() {
    let mut r = registers::Registers::new();

    r.set8(registers::REG_A, 12);
    println!("A: {}", r.get8(registers::REG_A));

    r.set_a( 220);
    println!("A: {}", r.get_a());

}
