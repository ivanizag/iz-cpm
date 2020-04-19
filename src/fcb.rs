use z80::machine::*;
use super::cpm_machine::*;

/*
File Control Block
See: http://www.gaby.de/cpm/manuals/archive/cpm22htm/ch5.htm

Fields:
    dr: drive code (0-16)
            0 = use default drive for file
            1 = auto disk select drive A,
            2 = auto disk select drive B,
            ...
            16 = auto disk select drive P.

    f1...f8: contain the filename in ASCII upper-case, with high bit = 0

    t1,t2,t3: contain the filetype in ASCII upper-case, with high bit = 0.
        t1', t2', and t3' denote the bit of these positions,
            t1' = 1 = Read-Only file,
            t2' = 1 = SYS file, no DIR list

    ex: contains the current extent number, normally set to 00 by the user,
        but in range 0-31 during file I/O

    s1: reserved for internal system use

    s2: reserved for internal system use, set to zero on call to OPEN, MAKE, SEARCH

    rc: record count for extent ex; takes on values from 0-127

    d0...d15: filled in by CP/M; reserved for system use

    cr: current record to read or write in a sequential file operation; normally
        set to zero by user

    r0,r1,r2: optional random record number in the range 0- 65535, with overflow
        to r2, r0, r1 constitute a 16-bit value with low byte r0, and high byte r1
*/
static FCB_NAME_OFFSET: u16 = 1;
static FCB_EXTENSION_OFFSET: u16 = 9;
static FCB_RANDOM_RECORD_OFFSET: u16 = 33;

pub struct Fcb<'a> {
    address: u16,
    machine: &'a CpmMachine
}

impl <'a> Fcb<'_> {
    pub fn new(address: u16, machine: &CpmMachine) -> Fcb {
        Fcb {
            address,
            machine
        }
    }

    //pub fn get_drive_code(&self) -> u8 {
    //    self.machine.peek(self.address)
    //}

    fn get_byte(&self, offset: u16) -> u8 {
        self.machine.peek(self.address + offset)
    }

    pub fn get_name(&self) -> String {
        let mut name = String::new();
        for i in 0..8 {
            let ch = self.get_byte(i + FCB_NAME_OFFSET) & 0x7F;
            name.push(ch as char)
        }
        name.push('.');
        for i in 0..3 {
            let ch = self.get_byte(i + FCB_EXTENSION_OFFSET) & 0x7F;
            name.push(ch as char)
        }
        name
    }

    pub fn get_random_record_number(&self) -> u32 {
        self.get_byte(FCB_RANDOM_RECORD_OFFSET) as u32
        + ((self.get_byte(FCB_RANDOM_RECORD_OFFSET + 1) as u32) << 8)
        + ((self.get_byte(FCB_RANDOM_RECORD_OFFSET + 2) as u32) << 16)
    }
}