use std::cell::RefCell;
use super::state::State;

pub trait Memory {
    fn peek(&self, address: u16) -> u8;
    fn poke(&mut self, address: u16, value: u8);

    fn peek16(&self, address: u16) -> u16 {
        self.peek(address) as u16
        + ((self.peek(address.wrapping_add(1)) as u16) << 8)
    }

    fn poke16(&mut self, address: u16, value: u16) {
        self.poke(address, value as u8 );
        self.poke(address.wrapping_add(1), (value >> 8) as u8);
    }
}

pub trait Io {
    fn port_in(&self, state: &State, address: u16) -> u8;
    fn port_out(&self, state: &State, address: u16, value: u8);
}

const PLAIN_MEMORY_SIZE: usize = 65536;
pub struct PlainMemoryIo {
    mem: [u8; PLAIN_MEMORY_SIZE],
    io: RefCell<[u8; PLAIN_MEMORY_SIZE]>
}

impl PlainMemoryIo {
    pub fn new() -> PlainMemoryIo {
        PlainMemoryIo {
            mem: [0; PLAIN_MEMORY_SIZE],
            io: RefCell::new([0; PLAIN_MEMORY_SIZE])
        }
    }
}

impl Memory for PlainMemoryIo {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}

impl Io for PlainMemoryIo {
    fn port_in(&self, _: &State, address: u16) -> u8 {
        self.io.borrow()[address as usize]
    }
    fn port_out(&self, _: &State, address: u16, value: u8) {
        self.io.borrow_mut()[address as usize] = value;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_byte() {
        let mut m = PlainMemoryIo::new();
        const A:u16 = 0x2345;
        const V:u8 = 0xa0;

        m.poke(A, V);
        assert_eq!(V, m.peek(A));
    }
}
