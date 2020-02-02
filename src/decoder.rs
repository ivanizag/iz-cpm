use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    name: &'static str,
    bytes: usize,
    cycles: u64,
    action: Box<OpcodeFn>,
}

impl Opcode {
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
            let mut opcode: Opcode;
            opcode = Opcode {
                name: "NOT IMPLEMENTED",
                bytes: 1,
                cycles: 4,
                action: Box::new(|_: &mut State| panic!("Not implemented"))
            };
            let p = DecodingHelper::parts(c);
            if p.x == 0 {
                if p.z == 0 {
                    // Relative jumps and assorted ops.
                    // TODO
                } else if p.z == 1 {
                    if p.q == 0 {
                        // 16-bit load immediate
                        //println!("Opcode LD 0x{:x} defined.", c);
                        opcode = Decoder::build_ld_rr_nn(p.p); // LD rp[p], nn -- 16-bit load add
                    } else {
                        // TODO
                    }
                } else if p.z == 2 {
                    // TODO
                } else if p.z == 3 {
                    // TODO
                } else if p.z == 4 {
                    if p.y == 6 {
                        // INC (HL) -- 8 bit inc
                        // TODO
                    } else {
                        //println!("Opcode INC 0x{:x} defined.", c);
                        opcode = Decoder::build_inc_r(p.y); // INC r[y] -- 8 bit inc
                    }
                } else if p.z == 5 {
                    // TODO
                } else if p.z == 6 {
                    // TODO
                } else if p.z == 7 {
                    // TODO
                }               
            } else if p.x == 1 {
                if p.z == 6 {
                    // TODO
                } else {
                    // TODO: HALT
                }
            }
            println!("0x{:x} {}: {:?}", c, opcode.name, p);
            self.no_prefix[c as usize] = Some(opcode)
        }
    }

    fn build_ld_rr_nn(p: usize) -> Opcode {
        let reg16 = TABLE_RP[p];
        Opcode {
            name: "LD rr, XX",
            bytes: 1,
            cycles: 10,
            action: Box::new(move |state: &mut State| {
                let value = state.advance_immediate();
                state.reg.set16(reg16, value);
                // TODO: flags
            })
        }
    }

    fn build_inc_r(y: usize) -> Opcode {
        let reg8 = TABLE_R[y];
        Opcode {
            name: "INC r",
            bytes: 1,
            cycles: 4,
            action: Box::new(move |state: &mut State| {
                let mut v = state.reg.get8(reg8);
                v = v + 1; // TODO: should wrap on 255
                state.reg.set8(reg8, v); 
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

const TABLE_RP: [usize; 4] = [REG_BC, REG_DE, REG_HL, REG_SP];
const TABLE_R:  [usize; 8] = [REG_B, REG_C, REG_D, REG_E, REG_H, REG_L, 0 /* (HL) not a register */, REG_A];

