
pub const REG_A: usize = 8;
pub const REG_COUNT8: usize = 14;

pub struct Registers {
    bytes: [u8; REG_COUNT8]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            bytes: [0; REG_COUNT8]
        }
    }

    pub fn get(&self, reg: usize) -> u8 {
        self.bytes[reg]
    }

    pub fn set(&mut self, reg: usize, value: u8) {
        self.bytes[reg] = value;
    }

    pub fn get_a(&self) -> u8 {
        self.get(REG_A)
    }

    pub fn set_a(&mut self, value: u8) {
        self.set(REG_A, value);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut r = Registers::new();
        const V:u8 = 23;

        r.set(REG_A, V);
        assert_eq!(V, r.get(REG_A));
    }

    #[test]
    fn set_get_a() {
        let mut r = Registers::new();
        const V:u8 = 44;

        r.set_a(V);
        assert_eq!(V, r.get_a());
    }


}