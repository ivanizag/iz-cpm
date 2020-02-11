use super::opcode::*;
use super::opcode_arith::*;
use super::opcode_bits::*;
use super::opcode_jumps::*;
use super::opcode_ld::*;
use super::registers::*;
use super::state::*;

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
        decoder.load_prefix_cb();
        decoder.load_prefix_ed();
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
            let p = DecodingHelper::parts(c);
            let opcode = match p.x {
                0 => match p.z {
                    0 => match p.y { // Relative jumps and assorted ops.
                        0 => Some(build_nop()), // NOP
                        1 => Some(build_ex_af()), // EX AF, AF'
                        2 => Some(build_djnz()), // DJNZ d
                        3 => Some(build_jr_unconditional()), // JR d
                        4..=7 => Some(build_jr_eq(CC[p.y-4])),
                        _ => panic!("Unreachable")
                    },
                    1 => match p.q {
                        0 =>  Some(build_ld_rr_nn(RP[p.p])), // LD rr, nn -- 16-bit load add
                        1 =>  Some(build_add_hl_rr(RP[p.p])), // ADD HL, rr -- 16-bit add
                        _ => panic!("Unreachable")
                    },
                    2 => match p.q {
                        0 =>  match p.p {
                            0 => Some(build_ld_prr_r(Reg16::BC, Reg8::A)), // LD (BC), A
                            1 => Some(build_ld_prr_r(Reg16::DE, Reg8::A)), // LD (DE), A
                            2 => Some(build_ld_pnn_rr(Reg16::HL)), // LD (nn), HL
                            3 => Some(build_ld_pnn_r(Reg8::A)), // LD (nn), A
                            _ => panic!("Unreachable")
                        },
                        1 =>  match p.p {
                            0 => Some(build_ld_r_prr(Reg8::A, Reg16::BC)), // LD A, (BC)
                            1 => Some(build_ld_r_prr(Reg8::A, Reg16::DE)), // LD A, (DE)
                            2 => Some(build_ld_rr_pnn(Reg16::HL)), // LD HL, (nn)
                            3 => Some(build_ld_r_pnn(Reg8::A)), // LD A, (nn)
                            _ => panic!("Unreachable")
                        }
                        _ => panic!("Unreachable")
                    },
                    3 => match p.q {
                        0 =>  Some(build_inc_dec_rr(RP[p.p], true)), // INC rr -- 16-bit inc
                        1 =>  Some(build_inc_dec_rr(RP[p.p], false)), // DEC rr -- 16-bit dec
                        _ => panic!("Unreachable")                       
                    },
                    4 => Some(build_inc_dec_r(R[p.y], true)), // INC r -- 8 bit inc
                    5 => Some(build_inc_dec_r(R[p.y], false)), // DEC r -- 8 bit dec
                    6 => Some(build_ld_r_n(R[p.y])), // LD r, n -- 8 bit load imm
                    7 => match p.y {
                        0..=3 => Some(build_rot_r(Reg8::A, ROT[p.y], true)), // rotA
                        4 => None, // DAA, decimal adjust A
                        5 => Some(build_cpl()), // CPL, complement adjust A
                        6 => Some(build_scf()), // SCF, set carry flag
                        7 => Some(build_ccf()), // CCF, clear carry flag
                        _ => panic!("Unreachable")
                    },
                    _ => panic!("Unreachable")
                },
                1 => match (p.z, p.y) {
                    (6, 6) => Some(build_halt()), // HALT, excetion instead of LD (HL), (HL)
                    _ => Some(build_ld_r_r(R[p.y], R[p.z], false)), // LD r[y], r[z] -- 8 bit load imm
                },
                2 => None,
                3 => match p.z {
                    0 => Some(build_ret_eq(CC[p.y])), // RET cc
                    1 => match p.q {
                        0 => Some(build_pop_rr(RP2[p.p])), // POP rr
                        1 => match p.p {
                            0 => Some(build_ret()), // RET
                            1 => Some(build_exx()), // EXX
                            2 => None, // JP HL
                            3 => Some(build_ld_rr_rr(Reg16::SP, Reg16::HL)), // LD SP, HL
                            _ => panic!("Unreachable")
                        },
                        _ => panic!("Unreachable")
                    },
                    2 => Some(build_jp_eq(CC[p.y])), // JP cc, nn
                    3 => match p.y {
                        0 => Some(build_jp_unconditional()), // JP nn
                        1 => None, // CB prefix
                        2 => None, // OUT
                        3 => None, // IN
                        4 => Some(build_ex_psp_rr(Reg16::HL)), // EX (SP), HL
                        5 => Some(build_ex_de_hl()), // EX DE, HL
                        6 => None, // DI
                        7 => None, // EI
                        _ => panic!("Unreachable")
                    }
                    4 => Some(build_call_eq(CC[p.y])),
                    5 => match p.q {
                        0 => Some(build_push_rr(RP2[p.p])), // PUSH rr
                        1 => match p.p {
                            0 => Some(build_call()), // Call nn
                            1 => None, // DD prefix
                            2 => None, // ED prefix
                            3 => None, // FD prefix
                            _ => panic!("Unreachable")
                        },
                        _ => panic!("Unreachable")
                    },
                    6 => None, // alu n
                    7 => Some(build_rst(p.y as u8 * 8)), // RST
                    _ => panic!("Unreachable")
                    },
                _ => panic!("Unreachable")
            };

            match opcode.as_ref() {
                None => println!("0x{:02x} {:20}: {:?}", c, "Pending", p),
                Some(o) => println!("0x{:02x} {:20}: {:?}", c, o.name, p)
            }
            self.no_prefix[c as usize] = opcode;
        }
    }

    fn load_prefix_cb(&mut self) {
        for c in 0..=255 {
            let p = DecodingHelper::parts(c);
            let opcode = match p.x {
                0 => Some(build_rot_r(R[p.z], ROT[p.y], false)), // Shifts
                1 => Some(build_bit_r(p.y as u8, R[p.z])), // BIT
                2 => Some(build_res_r(p.y as u8, R[p.z])), // RES
                3 => Some(build_set_r(p.y as u8, R[p.z])), // SET
                4..=7 => None, // Invalid instruction NONI + NOP
                _ => panic!("Unreachable")
            };

            match opcode.as_ref() {
                None => println!("0x{:02x} 0x{:02x} {:15}: {:?}", 0xcb, c, "Pending", p),
                Some(o) => println!("0x{:02x} 0x{:02x} {:15}: {:?}", 0xcb, c, o.name, p)
            }
            self.prefix_cb[c as usize] = opcode;
        }
    }

    fn load_prefix_ed(&mut self) {
        for c in 0..=255 {
            let p = DecodingHelper::parts(c);
            let opcode = match p.x {
                0 | 3 => Some(build_noni_nop()), // Invalid instruction NONI + NOP
                1 => match p.z {
                    0 => None,
                    1 => None,
                    2 => None,
                    3 => match p.q {
                        0 => Some(build_ld_pnn_rr(RP[p.p])), // LD (nn), rr -- 16 bit loading
                        1 => Some(build_ld_rr_pnn(RP[p.p])), // LD rr, (nn) -- 16 bit loading
                        _ => panic!("Unreachable")
                    },
                    4 => Some(build_neg()), // NEG
                    5 => None,
                    6 => None,
                    7 => match p.y {
                        0 => Some(build_ld_r_r(Reg8::I, Reg8::A, true)), // LD I, A
                        1 => Some(build_ld_r_r(Reg8::R, Reg8::A, true)), // LD R, A
                        2 => Some(build_ld_r_r(Reg8::A, Reg8::I, true)), // LD A, I
                        3 => Some(build_ld_r_r(Reg8::A, Reg8::R, true)), // LD A, R
                        4 => Some(build_rxd(ShiftDir::Right, "RRD")), // RRD
                        5 => Some(build_rxd(ShiftDir::Left, "RLD")),  // RLD
                        6 => Some(build_nop()), // NOP
                        7 => Some(build_nop()), // NOP
                        _ => panic!("Unreacheable")
                    },
                    _ => panic!("Unreacheable")
                },
                2 =>
                    if p.z <= 3 && p.y >= 4 {
                        None // bli[y,z] -- block instruction
                    } else {
                        Some(build_noni_nop()) // NONI + NOP
                    },
                _ => panic!("Unreachable")
            };

            match opcode.as_ref() {
                None => println!("0x{:02x} 0x{:02x} {:15}: {:?}", 0xed, c, "Pending", p),
                Some(o) => println!("0x{:02x} 0x{:02x} {:15}: {:?}", 0xed, c, o.name, p)
            }
            self.prefix_ed[c as usize] = opcode;
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


pub const RP:  [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];
pub const RP2: [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF];
pub const R:  [Reg8; 8] = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::_HL, Reg8::A];

pub const CC: [(Flag, bool, &'static str); 8] = [
    (Flag::Z, false, "NZ"),
    (Flag::Z, true,  "Z"),
    (Flag::C, false, "NC"),
    (Flag::C, true,  "C"),
    (Flag::P, false, "PO"),
    (Flag::P, true,  "PE"),
    (Flag::S, false, "P"),
    (Flag::S, true,  "N")
];

pub const ROT: [(ShiftDir, ShiftMode, &'static str); 8] = [
    (ShiftDir::Left,  ShiftMode::RotateCarry, "RLC"),
    (ShiftDir::Right, ShiftMode::RotateCarry, "RRC"),
    (ShiftDir::Left,  ShiftMode::Rotate,      "RL" ),
    (ShiftDir::Right, ShiftMode::Rotate,      "RR" ),
    (ShiftDir::Left,  ShiftMode::Arithmetic,  "SLA"),
    (ShiftDir::Right, ShiftMode::Arithmetic,  "SRA"),
    (ShiftDir::Left,  ShiftMode::Logical,     "SLL"),
    (ShiftDir::Right, ShiftMode::Logical,     "SRL"),
];