extern crate z80;

use z80::cpu::Cpu;
use z80::registers::Reg16;
use z80::registers::Reg8;
use z80::memory_io::*;


//static ZEXDOC: &'static [u8] = include_bytes!("res/zexdoc.com");
static ZEXALL: &'static [u8] = include_bytes!("res/zexall.com");


#[test]
fn text_zexall() {
    let mut machine = ZexMachine::new();
    let mut cpu = Cpu::new(&mut machine);

    // Load program
    //let code = ZEXDOC;
    let code = ZEXALL;
    let size = code.len();
    for i in 0..size {
        cpu.state.sys.poke(0x100 + i as u16, code[i]);
    }

    /*
    System call 5

    .org $5
        push af
        ld a, c
        out ($2), a
        ld a, d
        out ($3), a
        ld a, e
        out ($4), a
        in a, ($0)
        pull af
        ret

    F579D3027AD3037BD304DB00F1C9
    Compiled with http://clrhome.org/asm/

    */
    let code = [
        0xF5,
        0x79, 0xD3, 0x02,
        0x7A, 0xD3, 0x03,
        0x7B, 0xD3, 0x04,
        0xDB, 0x00,
        0xF1,
        0xC9];

    for i in 0..code.len() {
        cpu.state.sys.poke(5 + i as u16, code[i]);
    }

    /*
    Patch to have the stack where we need it
    We change:
        LD HL, (0006h)    // 2a 06 00
    to  LD HL, 0C900h      // 21 00 c9

    We have to put the bytes back afterwards.
    */
    cpu.state.sys.poke(0x0113, 0x21);
    cpu.state.sys.poke16(0x0114, 0xc900);

    // Patch to run a single test
    let run_single_test = false;
    let single_test = 11;
    if run_single_test {
        let mut test_start = cpu.state.sys.peek16(0x0120);
        test_start += single_test*2;
        cpu.state.sys.poke16(0x0120, test_start);
        cpu.state.sys.poke16(test_start + 2 , 0);
    
    }

    cpu.state.reg.set_pc(0x100);
    let trace = false;
    cpu.trace = trace;
    loop {
        cpu.execute_instruction();

        if trace {
            // CPU registers
            println!("PC({:04x}) AF({:04x}) BC({:04x}) DE({:04x}) HL({:04x}) SP({:04x}) IX({:04x}) IY({:04x}) Flags({:08b})",
                cpu.state.reg.get_pc(),
                cpu.state.reg.get16(Reg16::AF),
                cpu.state.reg.get16(Reg16::BC),
                cpu.state.reg.get16(Reg16::DE),
                cpu.state.reg.get16(Reg16::HL),
                cpu.state.reg.get16(Reg16::SP),
                cpu.state.reg.get16(Reg16::IX),
                cpu.state.reg.get16(Reg16::IY),
                cpu.state.reg.get8(Reg8::F)
            );

            // Test state
            let addr = 0x1d80 as u16;
            print!("Zex state 0x{:04x}: ", addr);
            for i in 0..0x10 {
                print!("{:02x} ", cpu.state.sys.peek(addr + i));
            }
            println!("");
        }

        if cpu.state.reg.get_pc() == 0x0116 {
            // Unpatch some code. The bytes are used on some tests
            cpu.state.sys.poke(0x0113, 0x2a);
            cpu.state.sys.poke16(0x0114, 0x0006);        
        }

        if cpu.state.reg.get_pc() == 0x0000 {
            println!("");
            break;
        }
    }

    if run_single_test {
        assert_eq!(1, machine.passed);
    } else {
        assert_eq!(67, machine.passed);
    }
}

pub struct ZexMachine {
    mem: [u8; PLAIN_MEMORY_SIZE],

    c: u8,
    d: u8,
    e: u8,

    passed: u8
}

impl ZexMachine {

    pub fn new() -> ZexMachine {
        ZexMachine {
            mem: [0; PLAIN_MEMORY_SIZE],
            c: 0, d: 0, e: 0,
            passed: 0
        }
    }

    fn bdos(&mut self) {
        match self.c {
            2 => self.bdos_c_write(),
            9 => self.bdos_c_write_str(),
            _ => panic!("BDOS command not implemented")
        }
    }

    fn bdos_c_write(&self) {
        print!("{}", self.e);
    }

    fn bdos_c_write_str(&mut self) {
        let mut address = ((self.d as u16) << 8) + self.e as u16;
        let mut msg = String::new();
        loop {
            let ch = self.peek(address) as char;
            address += 1;

            if ch == '$'{
                break;
            }
            msg.push(ch);
        }
        if msg.contains("OK") {
            self.passed += 1;
        }
        print!("{}", msg);
    }
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

