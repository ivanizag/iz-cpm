use iz80::Machine;

pub struct CpmMachine {
    mem: [u8; 65536],
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
            mem: [0; 65536],
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
        //println!("$$$ {:04x}", address);
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        //println!("$$$ {:04x} W", address);
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, address: u16) -> u8 {
        let value = self.in_values[address as u8 as usize];
        self.in_port = address as u8;
        self.in_called = true;
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        self.out_port = address as u8;
        self.out_value = value;
        self.out_called = true;
    }
}

