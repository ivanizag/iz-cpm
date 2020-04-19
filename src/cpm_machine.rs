use z80::machine::*;

pub struct CpmMachine {
    mem: [u8; PLAIN_MEMORY_SIZE],
    in_values: [u8; 256],
    in_called: bool,
    in_port: u8,
    out_called: bool,
    out_port: u8,
    out_value: u8
}

impl CpmMachine {
    pub fn new() -> CpmMachine {
        CpmMachine {
            mem: [0; PLAIN_MEMORY_SIZE],
            in_values: [0; 256],
            out_called: false,
            out_port: 0,
            out_value: 0,
            in_called: false,
            in_port: 0
        }
    }
}

impl Machine for CpmMachine {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, address: u16) -> u8 {
        let value = self.in_values[address as u8 as usize];
        if value != 1 {
            //print!("Port {:04x} in {:02x}\n", address, value);
        }
        self.in_port = address as u8;
        self.in_called = true;
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        //print!("Port {:04x} out {:02x} {}\n", address, value, value as char);
        self.out_port = address as u8;
        self.out_value = value;
        self.out_called = true;
    }
}

