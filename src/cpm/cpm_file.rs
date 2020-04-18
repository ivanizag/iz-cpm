use std::ffi::OsString;
use std::fs;
use std::io;

use super::fcb::*;

static DEFAULT_DMA: u16 = 0x0080;

pub struct CpmFile {
    dma: u16,
    file: Option<fs::File> // TODO: support more that one file opened
}

impl CpmFile {
    pub fn new() -> CpmFile {
        CpmFile {
            dma: DEFAULT_DMA,
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

    pub fn close(&self, _fcb: &Fcb) -> u8 {
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
        
        //TODO
        0
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