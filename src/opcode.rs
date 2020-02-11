use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    pub name: String,
    pub cycles: u64,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    fn new (name: String, cycles: u64, action: Box<OpcodeFn>) -> Opcode {
        Opcode {name, cycles, action}
    }

    pub fn execute(&self, state: &mut State) {
        (self.action)(state);
        state.cycles += self.cycles 
    }
}

pub fn build_nop() -> Opcode {
    Opcode {
        name: "NOP".to_string(),
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })
    }
}

pub fn build_noni_nop() -> Opcode {
    Opcode {
        name: "NONINOP".to_string(),
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })
    }
}

pub fn build_halt() -> Opcode {
    Opcode {
        name: "HALT".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.halted = true;
        })
    }
}

pub fn build_pop_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("POP {:?}", rr),
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.pop();
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_push_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("PUSH {:?}", rr),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get16(rr);
            state.push(value);
        })
    }
}
