use super::state::*;
use super::registers::*;

type OpcodeFn = fn(&mut State) -> ();

#[derive(Copy, Clone)]
pub struct Opcode {
    name: &'static str,
    bytes: usize,
    cycles: u64,
    action: OpcodeFn,
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
    unprefixed: [Opcode; 256],
    prefix_cb: [Opcode; 256],
    prefix_ed: [Opcode; 256],
    prefix_dd: [Opcode; 256],
    prefix_ddcb: [Opcode; 256],
    prefix_fd: [Opcode; 256],
    prefix_fdcb: [Opcode; 256],
}

impl Decoder {
    pub fn new() -> Decoder {
        let nop = Opcode {
            name: "NOP",
            bytes: 1,
            cycles: 4,
            action: |_: &mut State| ()
        };

        let mut decoder = Decoder {
            unprefixed: [nop; 256],
            prefix_cb: [nop; 256],
            prefix_ed: [nop; 256],
            prefix_dd: [nop; 256],
            prefix_ddcb: [nop; 256],
            prefix_fd: [nop; 256],
            prefix_fdcb: [nop; 256],
        };

        decoder.unprefixed[0x3c] = Opcode {
            name: "INC A",
            bytes: 1,
            cycles: 4,
            action: |state: &mut State| {
                let mut v = state.reg.get8(REG_A);
                v = v + 1;
                state.reg.set8(REG_A, v); 
            }
        };
        decoder
    }

    pub fn decode(&self, state: &mut State) -> &Opcode {
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
            _ => &self.unprefixed[b0 as usize]
            // TODO: verify how dddd, dded, ddfd, fddd, fded and fdfd work
        }
    }
}

/*
struct DecodingHelper {
    // See notation in http://www.z80.info/decoding.htm    
    x: u8,
    y: u8,
    z: u8,
    p: u8,
    q: u8
}

impl DecodingHelper {

}
*/