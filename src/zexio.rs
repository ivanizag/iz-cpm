use super::memory_io::*;

pub struct ZexMachine {
    mem: [u8; PLAIN_MEMORY_SIZE],

    step: u16,
    c: u8,
    d: u8,
    e: u8
}

impl Machine for ZexMachine {
    fn peek(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn port_in(&mut self, _address: u16) -> u8 {
        self.bdos();
        0
    }

    fn port_out(&mut self, address: u16, value: u8) {
        let port = address & 0xff;
        match port {
            2 => self.c = value,
            3 => self.d = value,
            4 => self.e = value,
            _ => {}
        }
    }
}

impl ZexMachine {

    pub fn new() -> ZexMachine {
        ZexMachine {
            mem: [0; PLAIN_MEMORY_SIZE],
            step: 0,
            c: 0, d: 0, e: 0
        }
    }


    fn bdos(&mut self) {
        match self.c {
            2 => self.bdos_c_write(),
            9 => self.bdos_c_writestr(),
            _ => panic!("BDOS command not implemented")
        }
    }

    fn bdos_c_write(&self) {
        print!("{}", self.e);
    }

    fn bdos_c_writestr(&mut self) {
        self.step += 1;
        let mut address = ((self.d as u16) << 8) + self.e as u16;
        let mut ch = self.peek(address) as char;
        //print!("<<");
        while ch != '$' {
            print!("{}", ch);
            address += 1;
            ch = self.peek(address) as char;
        }
        //print!(">>");
    }
}
