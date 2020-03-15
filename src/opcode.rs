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

    pub fn disasm(&self, state: &State) -> String {
        if self.name.contains("nn") {
            // Immediate argument 16 bits
            let nn = state.peek16_pc();
            let nn_str = format!("{:04x}h", nn);
            self.name.replace("nn", &nn_str)
        } else if self.name.contains("n") {
            // Immediate argument 8 bits
            let n = state.peek_pc();
            let n_str = format!("{:02x}h", n);
            self.name.replace("n", &n_str)
        } else if self.name.contains("n") {
            // Immediate argument 8 bits signed
            let d = state.peek_pc() as i8;
            let d_str = format!("{}", d);
            self.name.replace("d", &d_str)
        } else {
            self.name.clone()
        }
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

pub fn build_conf_interrupts(enable: bool) -> Opcode {
    let name = if enable {"EI"} else  {"DI"};
    Opcode {
        name: name.to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.reg.set_interrupts(enable);
        })
    }
}
