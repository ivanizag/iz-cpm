pub trait Machine {
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

    fn port_in(&mut self, address: u16) -> u8;
    fn port_out(&mut self, address: u16, value: u8);
}

pub const PLAIN_MEMORY_SIZE: usize = 65536;
pub struct PlainMachine {
    mem: [u8; PLAIN_MEMORY_SIZE],
    io: [u8; PLAIN_MEMORY_SIZE]
}

impl PlainMachine {
    pub fn new() -> PlainMachine {
        PlainMachine {
            mem: [0; PLAIN_MEMORY_SIZE],
            io: [0; PLAIN_MEMORY_SIZE]
        }
    }
}

impl Machine for PlainMachine {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, address: u16) -> u8 {
        self.io[address as usize]
    }
    fn port_out(&mut self, address: u16, value: u8) {
        self.io[address as usize] = value;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_byte() {
        let mut m = PlainMachine::new();
        const A:u16 = 0x2345;
        const V:u8 = 0xa0;

        m.poke(A, V);
        assert_eq!(V, m.peek(A));
    }
}
