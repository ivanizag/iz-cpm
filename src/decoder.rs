use std::num::Wrapping;

use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    name: String,
    bytes: usize,
    cycles: u64,
    action: Box<OpcodeFn>,
}

impl Opcode {
    fn new (name: String, bytes: usize, cycles: u64, action: Box<OpcodeFn>) -> Opcode {
        Opcode {name, bytes, cycles, action}
    }

    pub fn execute(&self, state: &mut State) {
        (self.action)(state);
        state.cycles += self.cycles 
    }
}

/* See
    http://www.z80.info/decoding.htm
    http://clrhome.org/table/
    http://z80-heaven.wikidot.com/instructions-set
*/

pub struct Decoder {
    no_prefix: [Option<Opcode>; 256],
    prefix_cb: [Option<Opcode>; 256],
    prefix_ed: [Option<Opcode>; 256],
    prefix_dd: [Option<Opcode>; 256],
    prefix_ddcb: [Option<Opcode>; 256],
    prefix_fd: [Option<Opcode>; 256],
    prefix_fdcb: [Option<Opcode>; 256],
}

impl Decoder {
    pub fn new() -> Decoder {

        let mut decoder = Decoder {
            no_prefix: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            prefix_cb: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            prefix_ed: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            prefix_dd: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            prefix_ddcb: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            prefix_fd: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            prefix_fdcb: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
        };
        decoder.load_no_prefix();
        decoder
    }

    pub fn decode(&self, state: &mut State) -> &Option<Opcode> {
        let b0 = state.advance_pc();
        match b0 {
            0xcb => &self.prefix_cb[state.advance_pc() as usize],
            0xed => &self.prefix_ed[state.advance_pc() as usize],
            0xdd => {
                let b1 = state.advance_pc();
                if b1 == 0xcb {
                    &self.prefix_ddcb[state.advance_pc() as usize]
                } else {
                    &self.prefix_dd[b1 as usize]
                }
            },
            0xfd => {
                let b1 = state.advance_pc();
                if b1 == 0xcb {
                    &self.prefix_fdcb[state.advance_pc() as usize]
                } else {
                    &self.prefix_fd[b1 as usize]
                }
            },
            _ => &self.no_prefix[b0 as usize]
            // TODO: verify how dddd, dded, ddfd, fddd, fded and fdfd work
        }
    }

    fn load_no_prefix(&mut self) {
        for c in 0..=255 {
            //let opcode: Option<Opcode>;
            let p = DecodingHelper::parts(c);
            let opcode = match p.x {
                0 => match p.z {
                    0 => match p.y { // Relative jumps and assorted ops.
                        0 => Some(Decoder::build_nop()), // NOP
                        1 => None,
                        2 => None,
                        3 => None,
                        4..=7 => None,
                        _ => panic!("Unreachable")
                    },
                    1 => match p.q { // 16 bit load imm / add 
                        0 =>  Some(Decoder::build_ld_rr_nn(p.p)), // LD rp[p], nn -- 16-bit load add
                        1 =>  None,
                        _ => panic!("Unreachable")
                    },
                    2 => None,
                    3 => match p.q {
                        0 =>  Some(Decoder::build_inc_dec_rr(p.p, true)), // INC rp[p] -- 16-bit inc
                        1 =>  Some(Decoder::build_inc_dec_rr(p.p, false)), // DEC rp[p] -- 16-bit dec
                        _ => panic!("Unreachable")                       
                    },
                    4 => match p.y { // 8 bit inc
                        6 => None, // INC (HL) -- 8 bit inc
                        0..=7 => Some(Decoder::build_inc_dec_r(p.y, true)), // INC r[y] -- 8 bit inc
                        _ => panic!("Unreachable")
                    },
                    5 => match p.y { // 8 bit dec
                        6 => None, // DEC (HL) -- 8 bit dec
                        0..=7 => Some(Decoder::build_inc_dec_r(p.y, false)), // DEC r[y] -- 8 bit dec
                        _ => panic!("Unreachable")
                    },
                    6 => None,
                    7 => None,
                    _ => panic!("Unreachable")
                },
                1 => None,
                2 => None,
                3 => None,
                4 => None,
                5 => None,
                6 => None,
                7 => None,
                _ => panic!("Unreachable")
            };

            match opcode.as_ref() {
                None => (),
                Some(o) => println!("0x{:02x} {:20}: {:?}", c, o.name, p)
            }
            self.no_prefix[c as usize] = opcode;
        }
    }

    fn build_nop() -> Opcode {
        Opcode {
            name: "NOP".to_string(),
            bytes: 1,
            cycles: 4,
            action: Box::new(|_: &mut State| {
                // Nothing done
            })

        }
    }

    fn build_ld_rr_nn(p: usize) -> Opcode {
        let reg16 = &TABLE_RP[p];
        Opcode {
            name: format!("LD {}, XX", TABLE_RP_NAME[p]),
            bytes: 1,
            cycles: 10,
            action: Box::new(move |state: &mut State| {
                let value = state.advance_immediate();
                state.reg.set16(reg16, value);
                // TODO: flags
            })
        }
    }

    fn build_inc_dec_rr(p: usize, inc: bool) -> Opcode {
        let reg16 = &TABLE_RP[p];
        let delta = if inc {1} else {65535};
        let mnemonic = if inc {"INC"} else {"DEC"};
        Opcode {
            name: format!("{} {}", mnemonic, TABLE_RP_NAME[p]),
            bytes: 1,
            cycles: 6,
            action: Box::new(move |state: &mut State| {
                let mut v = Wrapping(state.reg.get16(reg16));
                v = v + Wrapping(delta);
                state.reg.set16(reg16, v.0); 
                // TODO: flags
            })
        }    
    }    

    fn build_inc_dec_r(y: usize, inc: bool) -> Opcode {
        let reg8 = &TABLE_R[y];
        let delta = if inc {1} else {255};
        let mnemonic = if inc {"INC"} else {"DEC"};
        Opcode {
            name: format!("{} {}", mnemonic, TABLE_R_NAME[y]),
            bytes: 1,
            cycles: 4,
            action: Box::new(move |state: &mut State| {
                let mut v = Wrapping(state.reg.get8(reg8));
                v = v + Wrapping(delta);
                state.reg.set8(reg8, v.0); 
                // TODO: flags
            })
        }        
    }
}


#[derive(Debug)]
struct DecodingHelper {
    // See notation in http://www.z80.info/decoding.htm    
    x: usize,
    y: usize,
    z: usize,
    p: usize,
    q: usize
}

impl DecodingHelper {
    fn parts(code: u8) -> DecodingHelper {
        DecodingHelper {
            x: (code >> 6) as usize,
            y: ((code >> 3) & 7) as usize,
            z: (code & 7) as usize,
            p: ((code >> 4) & 3) as usize,
            q: ((code >> 3) & 1) as usize,
        }
    }
}

const TABLE_RP: [Register16; 4] = [
    Register16::BC, Register16::DE, Register16::HL, Register16::SP];
const TABLE_RP_NAME: [&str; 4] = [
    "BC", "DE", "HL", "SP"];
const TABLE_R:  [Register8; 8] = [
    Register8::B, Register8::C, Register8::D, Register8::E,
    Register8::H, Register8::L, Register8::_HL_, Register8::A];
const TABLE_R_NAME: [&str; 8] = [
    "B", "C", "D", "E",
    "H", "L", "undefined", "A"];

