use super::memory_io::Io;
use super::state::State;
use super::registers::*;

pub struct ZexIo {}

impl Io for ZexIo {
    fn port_in(&self, state: &State, _address: u16) -> u8 {
        //println!("IO address IN {:04x}", address);
        ZexIo::bdos(state);
        0xff
    }

    fn port_out(&self, _state: &State, _address: u16, _value: u8) {
        //println!("IO address OUT {:04x}: {:02x}", address, value);
    }
}

impl ZexIo {
    fn bdos(state: &State) {
        let f = state.reg.get8(Reg8::C);
        match f {
            2 => ZexIo::bdos_c_write(state),
            9 => ZexIo::bdos_c_writestr(state),
            _ => panic!("BDOS command not implemented")
        }
    }

    fn bdos_c_write(state: &State) {
        let ch = state.reg.get8(Reg8::E);
        print!("{}", ch);
    }

    fn bdos_c_writestr(state: &State) {
        let mut address = state.reg.get16(Reg16::DE);
        let mut ch = state.mem.peek(address) as char;
        while ch != '$' {
            print!("{}", ch);
            address += 1;
            ch = state.mem.peek(address) as char;
        }
    }
}
