use std::io::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use iz80::Machine;
use super::cpm_machine::*;

pub struct CpmConsole {
    stdin_channel: Receiver<u8>,
    next_char: Option<u8>
}

impl CpmConsole {
    pub fn new() -> CpmConsole {
        let (tx, rx) = mpsc::channel::<u8>();
        thread::spawn(move || loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            for mut c in buffer.bytes() {
                if c == 10 {c = 13};
                tx.send(c).unwrap();
            }
        });
        CpmConsole {
            stdin_channel: rx,
            next_char: None
        }
    }

    pub fn pool_keyboard(&mut self) {
        if self.next_char == None {
            self.next_char = match self.stdin_channel.try_recv() {
                Ok(key) => Some(key),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => panic!("Stdin disconnected")
            }
        }
    }

    pub fn read(&mut self) -> u8 {
        /*
        The Console Input function reads the next console character to
        register A. Graphic characters, along with carriage return,
        line- feed, and back space (CTRL-H) are echoed to the console.
        Tab characters, CTRL-I, move the cursor to the next tab stop. A
        check is made for start/stop scroll, CTRL-S, and start/stop
        printer echo, CTRL-P. The FDOS does not return to the calling
        program until a character has been typed, thus suspending
        execution if a character is not ready. 
        */

        match self.next_char {
            Some(ch) => {
                self.next_char = None;
                ch
            },
            None => {
                // Blocks waiting for char
                self.stdin_channel.recv().unwrap()
            }
        }
    }

    pub fn write(&self, ch: u8) {
        /*
        The ASCII character from register E is sent to the console
        device. As in Function 1, tabs are expanded and checks are made
        for start/stop scroll and printer echo. 
        */

        print!("{}", ch as char);
        stdout().flush().unwrap();
    }

    pub fn write_string(&self, address: u16, machine: &CpmMachine) {
        /*
        The Print String function sends the character string stored in
        memory at the location given by DE to the console device, until
        a $ is encountered in the string. Tabs are expanded as in
        Function 2, and checks are made for start/stop scroll and
        printer echo. 
        */

        let mut index = address;
        let mut msg = String::new();
        loop {
            let ch = machine.peek(index) as char;
            index += 1;
    
            if ch == '$'{
                break;
            }
            msg.push(ch);
        }
        print!("{}", msg);
        stdout().flush().unwrap();
    }

    pub fn read_string(&mut self, address: u16, machine: &mut CpmMachine) -> u8 {
        /*
        The Read Buffer function reads a line of edited console input
        into a buffer addressed by registers DE. Console input is
        terminated when either input buffer overflows or a carriage
        return or line-feed is typed. The Read Buffer takes the form:
            DE:	+0  +1  +2  +3  +4  +5  +6  +7  +8  ...  +n
	            mx  nc  cl  c2  c3  c4  c5  c6  c7  ...  ??
        where mx is the maximum number of characters that the buffer
        will hold, 1 to 255, and nc is the number of characters read
        (set by FDOS upon return) followed by the characters read from
        the console. If nc < mx, then uninitialized positions follow
        the last character, denoted by ?? in the above figure.

        TODO: Process controls characters
        */
        let max_size = machine.peek(address + 0);
        let mut size = 0;
        let mut pos = address + 2;
        loop {
            let ch = self.read();
            if ch == 10 || ch == 13 { // CR of LF
                break;
            }
            machine.poke(pos, ch);
            size += 1;
            pos += 1;
            if size >= max_size {
                // Buffer full
                break;
            }
        }

        machine.poke(address + 1, size);
        size
    }

    pub fn status(&self) -> u8 {
        /*
        The Console Status function checks to see if a character has
        been typed at the console. If a character is ready, the value
        0FFH is returned in register A. Otherwise a 00H value is returned. 
        */

        match self.next_char {
            Some(_) => 0xff,
            None => {
                // Avoid 100% CPU usage waiting for input.
                thread::sleep(Duration::from_nanos(100)); 
                0
            }
        }
    }

    pub fn raw_io(&mut self, data: u8) -> u8 {
        /*
        Direct Console I/O is supported under CP/M for those specialized
        applications where basic console input and output are required. Use
        of this function should, in general, be avoided since it bypasses
        all of the CP/M normal control character functions (for example,
        CTRL-S and CTRL-P). Programs that perform direct I/O through the
        BIOS under previous releases of CP/M, however, should be changed to
        use direct I/O under BDOS so that they can be fully supported under
        future releases of MP/M and CP/M.

        Upon entry to Function 6, register E either contains hexadecimal FF,
        denoting a console input request, or an ASCII character. If the
        input value is FF, Function 6 returns A = 00 if no character is
        ready, otherwise A contains the next console input character.

        If the input value in E is not FF, Function 6 assumes that E contains
        a valid ASCII character that is sent to the console.

        Function 6 must not be used in conjunction with other console I/O
        functions. 
        */

        if data == 0xff {
            // Input
            match self.next_char {
                Some(ch) => {
                    self.next_char = None;
                    ch
                },
                None => {
                    // Avoid 100% CPU usage waiting for input.
                    thread::sleep(Duration::from_millis(1)); 
                    0
                }
            }
        } else {
            // Output
            self.write(data);
            0 // Should this be 0 or data?
        }
    }
}