use super::decoder::*;
use super::memory::*;
use super::state::*;

pub struct Cpu {
    pub state: State,
    pub decoder: Decoder,
}

impl Cpu {
    pub fn new(memory: Box<dyn Memory>) -> Cpu {
        Cpu {
            state: State::new(memory),
            decoder: Decoder::new()
        }
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.decoder.decode(&mut self.state);
        opcode.execute(&mut self.state); 
    }
}
