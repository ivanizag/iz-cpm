use super::opcode::*;
use super::state::*;
use super::registers::*;

pub fn build_djnz() -> Opcode {
    Opcode {
        name: "DJNZ d".to_string(),
        bytes: 2,
        cycles: 8, // TODO: 13 if condition not met,
        action: Box::new(move |state: &mut State| {
            let offset = state.advance_pc();
            let b = state.reg.get8(Reg8::B).wrapping_add(0xff /* -1 */);
            state.reg.set8(Reg8::B, b);
            if b != 0 {
                // Condition not met
                relative_jump(state, offset);
            }
        })
    }
}

fn relative_jump(state: &mut State, offset: u8) {
    let mut pc = state.reg.get_pc();
    pc = pc.wrapping_add(offset as i8 as i16 as u16);
    pc = pc.wrapping_add(-2 as i16 as u16); // Assume rel jump opcode took 2 bytes
    state.reg.set_pc(pc);
}

