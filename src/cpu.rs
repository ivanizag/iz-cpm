use super::decoder::*;
use super::memory_io::*;
use super::state::*;

pub struct Cpu {
    pub state: State,
    pub decoder: Decoder,
}

impl Cpu {
    pub fn new(sys: Box<dyn Machine>) -> Cpu {
        Cpu {
            state: State::new(sys),
            decoder: Decoder::new()
        }
    }

    pub fn new_plain() -> Cpu {
        let sys = Box::new(PlainMachine::new());
        Cpu {
            state: State::new(sys),
            decoder: Decoder::new()
        }
    }

    pub fn execute_instruction(&mut self) {
        let trace = false;
        if trace {
            let pc = self.state.reg.get_pc();
            let opcode_index = self.state.sys.peek(pc);
            //print!("==== {:04x}: {:02x} ", pc, opcode_index);
            print!("==== {:04x}: {:02x} {:02x} {:02x} ", pc, opcode_index,
                self.state.sys.peek(pc+1), self.state.sys.peek(pc+2));
        }
        let opcode = self.decoder.decode(&mut self.state);
        if trace {
            println!("{}", opcode.disasm(&self.state));
        }
        opcode.execute(&mut self.state);
        self.state.step();
    }
}


