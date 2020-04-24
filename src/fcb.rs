use iz80::Machine;
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

*/
const FCB_NAME_OFFSET: u16 = 1;
/*
    f1...f8: contain the filename in ASCII upper-case, with high bit = 0
*/
const FCB_EXTENSION_OFFSET: u16 = 9;
/*
    t1,t2,t3: contain the filetype in ASCII upper-case, with high bit = 0.
        t1', t2', and t3' denote the bit of these positions,
            t1' = 1 = Read-Only file,
            t2' = 1 = SYS file, no DIR list
*/
const FCB_EXTENT_OFFSET: u16 = 12;
/*
    ex: contains the current extent number, normally set to 00 by the user,
        but in range 0-31 during file I/O
    s1: reserved for internal system use
    s2: reserved for internal system use, set to zero on call to OPEN, MAKE, SEARCH
*/
const FCB_RECORD_COUNT_OFFSET: u16 = 15;
/*
    rc: record count for extent ex; takes on values from 0-127
*/
//const FCB_INTERNAL_OFFSET: u16 = 16;
/*
    d0...d15: filled in by CP/M; reserved for system use
*/
const FCB_CURRENT_RECORD_OFFSET: u16 = 32;
/*
    cr: current record to read or write in a sequential file operation; normally
        set to zero by user
*/
const FCB_RANDOM_RECORD_OFFSET: u16 = 33;
/*
    r0,r1,r2: optional random record number in the range 0- 65535, with overflow
        to r2, r0, r1 constitute a 16-bit value with low byte r0, and high byte r1
*/

const EXTENT_SIZE: u8 = 128; // We will assume all extent have 128 records.

pub struct Fcb<'a> {
    address: u16,
    machine: &'a mut CpmMachine
}

impl <'a> Fcb<'_> {
    pub fn new(address: u16, machine: &mut CpmMachine) -> Fcb {
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

    fn set_byte(&mut self, offset: u16, v: u8) {
        self.machine.poke(self.address + offset, v)
    }

    pub fn init(&mut self) {
        self.set_byte(FCB_EXTENT_OFFSET, 0);
        self.set_byte(FCB_CURRENT_RECORD_OFFSET, 0);
        self.set_byte(FCB_RECORD_COUNT_OFFSET, 0);
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

    /*
    pub fn get_name_secondary(&self) -> String {
        let mut name = String::new();
        for i in 0..8 {
            let ch = self.get_byte(i + FCB_NAME_OFFSET + FCB_INTERNAL_OFFSET) & 0x7F;
            name.push(ch as char)
        }
        name.push('.');
        for i in 0..3 {
            let ch = self.get_byte(i + FCB_EXTENSION_OFFSET + FCB_INTERNAL_OFFSET) & 0x7F;
            name.push(ch as char)
        }
        name
    }
    */

    pub fn set_name(&mut self, name: String) {
        let bytes = name.as_bytes();
        for i in 0..8 {
            self.set_byte(i + FCB_NAME_OFFSET, 0x7F & bytes[i as usize]);
        }
        for i in 0..3 {
            self.set_byte(i + FCB_EXTENSION_OFFSET, 0x7F & bytes[4 + i as usize]);
        }
    }

    pub fn get_name_host(&self) -> String {
        let mut name = String::new();
        for i in 0..8 {
            let ch = self.get_byte(i + FCB_NAME_OFFSET) & 0x7F;
            if ch == ' ' as u8 {
                break;
            }
            name.push(ch as char)
        }
        name.push('.');
        for i in 0..3 {
            let ch = self.get_byte(i + FCB_EXTENSION_OFFSET) & 0x7F;
            if ch == ' ' as u8 {
                break;
            }
            name.push(ch as char)
        }
        name
    }

    pub fn get_sequential_record_number(&self) -> u16 {
        (EXTENT_SIZE as u16) * (self.get_byte(FCB_EXTENT_OFFSET) as u16)
        + (self.get_byte(FCB_CURRENT_RECORD_OFFSET) as u16)
    }

    pub fn inc_current_record(&mut self) {
        let cr = 1 + self.get_byte(FCB_CURRENT_RECORD_OFFSET);
        if cr == EXTENT_SIZE {
            self.set_byte(FCB_CURRENT_RECORD_OFFSET, 0);
            self.set_byte(FCB_EXTENT_OFFSET,
                1 + self.get_byte(FCB_EXTENT_OFFSET)); 
        } else {
            self.set_byte(FCB_CURRENT_RECORD_OFFSET, cr);
        }
    }

    pub fn get_random_record_number(&self) -> u32 {
        self.get_byte(FCB_RANDOM_RECORD_OFFSET) as u32
        + ((self.get_byte(FCB_RANDOM_RECORD_OFFSET + 1) as u32) << 8)
        + ((self.get_byte(FCB_RANDOM_RECORD_OFFSET + 2) as u32) << 16)
    }


}



/*
The characters used in specifying an unambiguous file reference cannot contain
any of the following special characters:
    < > . , ; : = ? * [ ] % | ( ) / \
while all alphanumerics and remaining special characters are allowed.

CCP parses the command line to extract the name of the program to run, and one
or two additional filenames. To the CCP, the following characters are not valid
for use in filenames:
    space = _ . : ; < >

The CPM CCP module converts commands into upper case before they are executed
which leads many to believe that the CPM file system is not case sensitive, when
in fact the CPM file system is case sensitive. If you use a CPM program such
as Microsoft Basic you can create file names which contain lower case characters.
The problem is files which contain lower case characters can not be specified as
parameters at the CCP command prompt, as the characters will be converted to upper
case by the CCP before the command is executed.
*/
pub fn name_to_8_3(os_name: &str) -> Option<String> {
    let mut name = String::new();
    let mut extension = String::new();
    let mut in_extension = false;
    for ch in os_name.chars() {
        if !ch.is_ascii() {
            return None; // Only ascii chars allowed.
        }
        // Note: let's not change to upper case. We may need to review this.
        //ch = ch.to_ascii_uppercase();
        if !in_extension {
            if ch == '.' {
                in_extension = true;
            } else {
                name.push(ch);
            }
        } else {
            if ch == '.' {
                return None; // Only one dot allowed.
            } else {
                extension.push(ch);
            }
        }
    }

    // Verify it fits in 8 + 3
    if name.len() > 8 || extension.len() > 3 {
        return None;
    }

    // Pad with spaces and compose
    Some(format!("{:8}.{:3}", name, extension))
}

/*
An ambiguous file reference is used for directory search and pattern matching.
The form of an ambiguous file reference is similar to an unambiguous reference,
except the symbol ? can be interspersed throughout the primary and secondary
names. In various commands throughout CP/M, the ? symbol matches any character
of a filename in the ? position. Thus, the ambiguous reference "X?Z.C?M" matches
the following unambiguous filenames "XYZ.COM" and "X3Z.CAM".
The wildcard character can also be used in an ambiguous file reference. The *
character replaces all or part of a filename or filetype. Note that "*.*" equals
the ambiguous file reference "????????.???" while "filename.*" and "*.typ" are
abbreviations for "filename.???" and "????????.typ" respectively.
*/
const WILDCARD: u8 = '?' as u8;
pub fn name_match(name: &str, pattern: &str) -> bool {
    let n = name.as_bytes();
    let p = pattern.as_bytes();
    for i in 0..(8+1+3) {
        if (n[i] != p[i]) || (p[i] != WILDCARD) {
            return false;
        }
    }
    true
}