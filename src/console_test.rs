use core::panic;
use std::collections::VecDeque;

use crate::console_emulator::ConsoleEmulator;

pub enum Step <'a> {
    Input(&'a str),
    Expect(&'a str),
    ExpectTimeout(&'a str, u32),
}

/*
When Input is found, it is added to the input queue. We wait for this to be consumed ignoring
the output. When completed, we move to the next step.

When Expect is found, we collect the output until the expected output is found as a substring.
We wait at most the nuber give of calls to status(). When found, we move to the next step.

When all the steps are completed, the test is passed.
*/


pub struct ConsoleTest <'a> {
    input: VecDeque<u8>,
    expected_output: Option<&'a str>,
    current_output: String,
    current_count_left: u32,

    script: Vec<Step<'a>>,
    step: usize,
    terminate: bool,
}

impl <'a> ConsoleTest <'a> {
    pub fn new(mut script: Vec<Step>) -> ConsoleTest {
        script.reverse(); // Reverse to pop from the end

        let mut c = ConsoleTest {
            input : VecDeque::new(),
            expected_output: None,
            current_output: String::new(),
            current_count_left: 0,

            script,
            step: 0,
            terminate: false,
        };

        c.next_step();
        c
    }

    fn next_step(&mut self) {
        self.input.clear();
        self.expected_output = None;
        self.current_output.clear();
        self.current_count_left = 0;

        let step = self.script.pop();
        self.step += 1;
        match step {
            Some(Step::Input(input)) => {
                self.input = input.chars().map(|c| c as u8).collect();            }
            Some(Step::Expect(output)) => {
                self.expected_output = Some(output);
                self.current_count_left = 100; // 100 calls to status() by default
            }
            Some(Step::ExpectTimeout(output, wait)) => {
                self.expected_output = Some(output);
                self.current_count_left = wait;
            }
            None => {
                self.terminate = true;
            }
        }
    }
}

impl<'a> ConsoleEmulator for ConsoleTest <'a> {
    fn status(&mut self) -> bool {
        if self.input.len() > 0 {
            true
        } else {
            if self.current_count_left == 0 {
                panic!("Test failed in step {}: expected text not found", self.step);
            }
            self.current_count_left -= 1;
            false
        }
    }

    fn read(&mut self) -> u8 {
        match self.input.pop_front() {
            Some(ch) => {
                if self.input.len() == 0 {
                    self.next_step();
                }
                ch
            }
            None => {
                panic!("Test failed in step {}: input not available waiting for expected text", self.step);
            }
        }
    }

    fn put(&mut self, sequence: Option<String>) {
        if let Some(sequence) = sequence {
            print!("{}", sequence);

            if let Some(expected) = &self.expected_output {
                self.current_output.push_str(&sequence);
                if self.current_output.contains(*expected) {
                    self.next_step();
                }
            }
        }
    }

    fn terminated(&self) -> bool {
        self.terminate
    }
}

