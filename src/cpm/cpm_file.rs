use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;

use z80::memory_io::*;

use super::fcb::*;
use super::cpm_machine::*;

const DEFAULT_DMA: u16 = 0x0080;
const RECORD_SIZE: usize = 128;

pub struct CpmFile {
    dma: u16,
    buffer: [u8; RECORD_SIZE],
    file: Option<fs::File> // TODO: support more that one file opened
}

impl CpmFile {
    pub fn new() -> CpmFile {
        CpmFile {
            dma: DEFAULT_DMA,
            buffer: [0; RECORD_SIZE],
            file: None
        }
    }

    pub fn reset(&mut self) {
        /*
        The Reset Disk function is used to programmatically restore the
        file system to a reset state where all disks are set to
        Read-Write. See functions 28 and 29, only disk drive A is
        selected, and the default DMA address is reset to BOOT+0080H.
        This function can be used, for example, by an application
        program that requires a disk change without a system reboot.
        */
        // TODO
        //cpm.disk.reset()
        self.dma = DEFAULT_DMA;
    }

    pub fn set_dma(&mut self, dma: u16) {
        /*
        DMA is an acronym for Direct Memory Address, which is often used
        in connection with disk controllers that directly access the
        memory of the mainframe computer to transfer data to and from the
        disk subsystem. Although many computer systems use non-DMA access
        (that is, the data is transferred through programmed I/O
        operations), the DMA address has, in CP/M, come to mean the
        address at which the 128-byte data record resides before a disk
        write and after a disk read. Upon cold start, warm start, or disk
        system reset, the DMA address is automatically set to BOOT+0080H.
        The Set DMA function can be used to change this default value to
        address another area of memory where the data records reside.
        Thus, the DMA address becomes the value specified by DE until it
        is changed by a subsequent Set DMA function, cold start, warm
        start, or disk system reset. 
        */
        self.dma = dma;
    }

    pub fn open(&mut self, fcb: &Fcb) -> u8 {
        /*
        The Open File operation is used to activate a file that currently
        exists in the disk directory for the currently active user number.
        The FDOS scans the referenced disk directory for a match in
        positions 1 through 14 of the FCB referenced by DE (byte s1 is
        automatically zeroed) where an ASCII question mark (3FH) matches
        any directory character in any of these positions. Normally, no
        question marks are included, and bytes ex and s2 of the FCB are
        zero.

        If a directory element is matched, the relevant directory
        information is copied into bytes d0 through dn of FCB, thus
        allowing access to the files through subsequent read and write
        operations. The user should note that an existing file must not
        be accessed until a successful open operation is completed. Upon
        return, the open function returns a directory code with the
        value 0 through 3 if the open was successful or 0FFH (255 decimal)
        if the file cannot be found. If question marks occur in the FCB,
        the first matching FCB is activated. Note that the current record,
        (cr) must be zeroed by the program if the file is to be accessed
        sequentially from the first record. 
        */

        //TODO
        let path = self.find_host_file(fcb.get_name());
        match path {
            Err(_) => 0xff, // Error or File not found
            Ok(path) => {
                let file = fs::File::open(path).unwrap();
                self.file = Some(file);
                0
            }
        }
    }

    pub fn close(&mut self, _fcb: &Fcb) -> u8 {
        /*
        The Close File function performs the inverse of the Open File
        function. Given that the FCB addressed by DE has been previously
        activated through an open or make function, the close function
        permanently records the new FCB in the reference disk directory
        (see functions 15 and 22). The FCB matching process for the close
        is identical to the open function. The directory code returned for
        a successful close operation is 0, 1, 2, or 3, while a 0FFH (255
        decimal) is returned if the filename cannot be found in the
        directory. A file need not be closed if only read operations have
        taken place. If write operations have occurred, the close operation
        is necessary to record the new directory information permanently. 
        */
        
        match &self.file {
            None => 0xff,
            Some(_) => {
                self.file = None; 
                0
            }
        }
    }

    pub fn read_rand(&mut self, fcb: &Fcb) -> u8 {
        /*
        The Read Random function is similar to the sequential file read
        operation of previous releases, except that the read operation
        takes place at a particular record number, selected by the 24-bit
        value constructed from the 3-byte field following the FCB (byte
        positions r0 at 33, r1 at 34, and r2 at 35). The user should note
        that the sequence of 24 bits is stored with least significant byte
        first (r0), middle byte next (r1), and high byte last (r2). CP/M
        does not reference byte r2, except in computing the size of a file
        (see Function 35). Byte r2 must be zero, however, since a nonzero
        value indicates overflow past the end of file.

        Thus, the r0, r1 byte pair is treated as a double-byte, or word
        value, that contains the record to read. This value ranges from 0
        to 65535, providing access to any particular record of the
        8-megabyte file. To process a file using random access, the base
        extent (extent 0) must first be opened. Although the base extent
        might or might not contain any allocated data, this ensures that
        the file is properly recorded in the directory and is visible in
        DIR requests. The selected record number is then stored in the
        random record field (r0, r1), and the BDOS is called to read the
        record.

        Upon return from the call, register A either contains an error
        code, as listed below, or the value 00, indicating the operation
        was successful. In the latter case, the current DMA address
        contains the randomly accessed record. Note that contrary to the
        sequential read operation, the record number is not advanced.
        Thus, subsequent random read operations continue to read the same
        record.

        Upon each random read operation, the logical extent and current
        record values are automatically set. Thus, the file can be
        sequentially read or written, starting from the current randomly
        accessed position. However, note that, in this case, the last
        randomly read record will be reread as one switches from random
        mode to sequential read and the last record will be rewritten as
        one switches to a sequential write operation. The user can simply
        advance the random record position following each random read or
        write to obtain the effect of sequential I/O operation.

        Error codes returned in register A following a random read are
        listed below.
            01	reading unwritten data
            02	(not returned in random mode)
            03	cannot close current extent
            04	seek to unwritten extent
            05	(not returned in read mode)
            06	seek Past Physical end of disk

        Error codes 01 and 04 occur when a random read operation
        accesses a data block that has not been previously written or an
        extent that has not been created, which are equivalent
        conditions. Error code 03 does not normally occur under proper
        system operation. If it does, it can be cleared by simply
        rereading or reopening extent zero as long as the disk is not
        physically write protected. Error code 06 occurs whenever byte
        r2 is nonzero under the current 2.0 release. Normally, nonzero
        return codes can be treated as missing data, with zero return
        codes indicating operation complete. 
        */
        match &mut self.file {
            None => 4, //04 file is not opened
            Some(os_file) => {
                let record = fcb.get_random_record_number();
                if record > 65535 {
                    return 6; //06	seek Past Physical end of disk
                }

                let file_offset = record as u64 * RECORD_SIZE as u64;
                let res = os_file.seek(io::SeekFrom::Start(file_offset));
                if let Err(_) = res {
                    return 6; //06	seek Past Physical end of disk
                }

                let res = os_file.read(&mut self.buffer);
                let size = match res {
                    Ok(n) => n,
                    _ => return 1 // 01 reading unwritten data
                };

                // Fill with zeros
                for i in size..RECORD_SIZE {
                    self.buffer[i] = 0;
                }
                0
            }
        }
    }

    pub fn load_buffer(&self, machine: &mut CpmMachine) {
        for i in 0..RECORD_SIZE {
            machine.poke(self.dma + i as u16, self.buffer[i]);
        }
    }

    fn find_host_file(&self, name: String) -> io::Result<OsString> {
        let dir = fs::read_dir("./")?;
        for entry in dir {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let cpm_name = name_to_8_3(&entry.file_name().to_string_lossy());
                if let Some(s) = cpm_name {
                    if s == name {
                        // File found
                        return Ok(entry.path().into_os_string())
                    }
                }

                /*match cpm_name {
                    Some(name) => print!("Name '{}' is valid\n", name),
                    None => print!("Name '{}' is not valid\n", entry.file_name().to_string_lossy())
                }*/
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, ""))
    }
}

/*
The characters used inspecifying an unambiguous file reference cannot contain
any of the following special characters:
    < > . , ; : = ? * [ ] % | ( ) / \
while all alphanumerics and remaining special characters are allowed.

CCP parses the command line to extract the name of the program to run, and one
or two additional filenames. To the CCP, the following characters are not valid
for use in filenames:
    space = _ . : ; < >

The CPM CPP module converts commands into upper case before they are executed
which leads many to believe that the CPM file system is not case sensitive, when
in fact the CPM file system is case sensitive. If you use a CPM program such
as Microsoft Basic you can create file names which contain lower case characters.
The problem is files which contain lower case characters can not be specified as
parameters at the CPP command prompt, as the characters will be converted to upper
case by the CPP before the command is executed.
*/
fn name_to_8_3(os_name: &str) -> Option<String> {
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