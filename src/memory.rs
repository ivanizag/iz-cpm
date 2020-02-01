
pub trait Memory {
    fn peek(&self, address: u16) -> u8;
    fn poke(&mut self, address: u16, value: u8);
}

const PLAIN_MEMORY_SIZE: usize = 65536;
pub struct PlainMemory {
    bytes: [u8; 65536]
}

impl PlainMemory {
    pub fn new() -> PlainMemory {
        PlainMemory {
            bytes: [0; PLAIN_MEMORY_SIZE]
        }
    }
}

impl Memory for PlainMemory {
    fn peek(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }
    fn poke(&mut self, address: u16, value: u8) {
        self.bytes[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_byte() {
        let mut m = PlainMemory::new();
        const A:u16 = 0x2345;
        const V:u8 = 0xa0;

        m.poke(A, V);
        assert_eq!(V, m.peek(A));
    }
}
