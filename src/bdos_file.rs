use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

use super::bdos_environment::*;
use super::fcb::*;

// Many file processing functions return a value in register A that is either
// OFFH, indicating that the file named in the FCB could not be found, or equal
// to a value of 0, 1, 2, or 3. In the latter case, the BDOS is returning what
// is called a "directory code." The number is the directory entry number that
// the BDOS matched to the filename in your FCB. At any given moment, the BDOS
// has a 128-byte sector from the directory in memory. Each filed irectory entry
// is 32 bytes, so four of them (numbered 0, 1,2, and 3) can be processed at a
// time. The directory code indicates which one has been matched to your
// FCB.alloc
// Here we will have always the files in directory code 0.
const DIRECTORY_CODE: u8 = 0;
const FILE_NOT_FOUND: u8 = 0xff;

pub fn set_dma(env: &mut BdosEnvironment, dma: u16) {
    // DMA is an acronym for Direct Memory Address, which is often used in
    // connection with disk controllers that directly access the memory of the
    // mainframe computer to transfer data to and from the disk subsystem.
    // Although many computer systems use non-DMA access (that is, the data is
    // transferred through programmed I/O operations), the DMA address has, in
    // CP/M, come to mean the address at which the 128-byte data record resides
    // before a disk write and after a disk read. Upon cold start, warm start,
    // or disk system reset, the DMA address is automatically set to BOOT+0080H.
    // The Set DMA function can be used to change this default value to address
    // another area of memory where the data records reside. Thus, the DMA
    // address becomes the value specified by DE until it is changed by a
    // subsequent Set DMA function, cold start, warm start, or disk system
    // reset. 
    env.state.dma = dma;
}

pub fn open(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Open File operation is used to activate a file that currently exists
    // in the disk directory for the currently active user number. The FDOS
    // scans the referenced disk directory for a match in positions 1 through 14
    // of the FCB referenced by DE (byte s1 is automatically zeroed) where an
    // ASCII question mark (3FH) matches any directory character in any of these
    // positions. Normally, no question marks are included, and bytes ex and s2
    // of the FCB are zero.
    // If a directory element is matched, the relevant directory information is
    // copied into bytes d0 through dn of FCB, thus allowing access to the files
    // through subsequent read and write operations. The user should note that
    // an existing file must not be accessed until a successful open operation
    // is completed. Upon return, the open function returns a directory code
    // with the value 0 through 3 if the open was successful or 0FFH (255
    // decimal) if the file cannot be found. If question marks occur in the FCB,
    // the first matching FCB is activated. Note that the current record, (cr)
    // must be zeroed by the program if the file is to be accessed sequentially
    // from the first record. 
    let mut fcb = Fcb::new(fcb_address, env.machine);
    if env.call_trace {
        print!("[[Open file {}]]", fcb.get_name());
    }
    match find_host_files(fcb.get_name(), false) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(paths) => match fs::File::open(&paths[0]) {
            Err(_) => FILE_NOT_FOUND,
            Ok(_) => {
                fcb.init();
                DIRECTORY_CODE
            }
        }
    }
}

pub fn make(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Make File operation is similar to the Open File operation except that
    // the FCB must name a file that does not exist in the currently referenced
    // disk directory (that is, the one named explicitly by a nonzero dr code or
    // the default disk if dr is zero). The FDOS creates the file and
    // initializes both the directory and main memory value to an empty file.
    // The programmer must ensure that no duplicate filenames occur, and a
    // preceding delete operation is sufficient if there is any possibility of
    // duplication. Upon return, register A = 0, 1, 2, or 3 if the operation was
    // successful and 0FFH (255 decimal) if no more directory space is
    // available. The Make function has the side effect of activating the FCB
    // and thus a subsequent open is not necessary.
    let mut fcb = Fcb::new(fcb_address, env.machine);
    if env.call_trace {
        print!("[[Create file {}]]", fcb.get_name());
    }
    match fs::File::create(fcb.get_name_host()) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(_) => {
            fcb.init();
            DIRECTORY_CODE
        }
    }
}

pub fn close(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Close File function performs the inverse of the Open File function.
    // Given that the FCB addressed by DE has been previously activated through
    // an open or make function, the close function permanently records the new
    // FCB in the reference disk directory (see functions 15 and 22). The FCB
    // matching process for the close is identical to the open function. The
    // directory code returned for a successful close operation is 0, 1, 2, or
    // 3, while a 0FFH (255 decimal) is returned if the filename cannot be found
    // in the directory. A file need not be closed if only read operations have
    // taken place. If write operations have occurred, the close operation is
    // necessary to record the new directory information permanently. 
    let fcb = Fcb::new(fcb_address, env.machine);
    match find_host_files(fcb.get_name(), false){
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(_) => DIRECTORY_CODE
    }
}

pub fn delete(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Delete File function removes files that match the FCB addressed by
    // DE. The filename and type may contain ambiguous references (that is,
    // question marks in various positions), but the drive select code cannot be
    // ambiguous, as in the Search and Search Next functions.
    // Function 19 returns a decimal 255 if the referenced file or files cannot
    // be found; otherwise, a value in the range 0 to 3 returned.
    let fcb = Fcb::new(fcb_address, env.machine);
    if env.call_trace {
        print!("[[Delete file {}]]", fcb.get_name());
    }

    match find_host_files(fcb.get_name(), true) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(paths) => {
            for name in paths {
                if fs::remove_file(name).is_err() {
                    return FILE_NOT_FOUND;
                }
            }
            DIRECTORY_CODE
        }
    }
}

pub fn rename(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Rename function uses the FCB addressed by DE to change all
    // occurrences of the file named in the first 16 bytes to the file named in
    // the second 16 bytes. The drive code dr at postion 0 is used to select the
    // drive, while the drive code for the new filename at position 16 of the
    // FCB is assumed to be zero. Upon return, register A is set to a value
    // between 0 and 3 if the rename was successful and 0FFH (255 decimal) if
    // the first filename could not be found in the directory scan. 
    let fcb = Fcb::new(fcb_address, env.machine);
    if env.call_trace {
        print!("[[Rename file {} to {}]]", fcb.get_name(), fcb.get_name_secondary());
    }
    match find_host_files(fcb.get_name(), false) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(paths) => {
            for name in paths {
                let new_name = name_from_8_3(&fcb.get_name_secondary());
                if fs::rename(name, new_name).is_err() {
                    return FILE_NOT_FOUND;
                }
            }
            DIRECTORY_CODE
        }
    }
}

pub fn read(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // Given that the FCB addressed by DE has been activated through an Open or
    // Make function, the Read Sequential function reads the next 128-byte
    // record from the file into memory at the current DMA address. The record
    // is read from position cr of the extent, and the cr field is automatically
    // incremented to the next record position. If the cr field overflows, the
    // next logical extent is automatically opened and the cr field is reset to
    // zero in preparation for the next read operation. The value 00H is
    // returned in the A register if the read operation was successful, while a
    // nonzero value is returned if no data exist at the next record position
    // (for example, end-of-file occurs).
    let mut fcb = Fcb::new(fcb_address, env.machine);
    if env.call_trace {
        print!("[Read record {:x} into {:04x}]",
            fcb.get_sequential_record_number(), env.state.dma);
    }

    let record = fcb.get_sequential_record_number();
    fcb.inc_current_record();
    let res = read_record_in_buffer(&mut env.state.buffer, &fcb, record as u16).unwrap_or(1);
    if res == DIRECTORY_CODE {
        env.load_buffer();
    }
    res
}

pub fn write(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // Given that the FCB addressed by DE has been activated through an Open or
    // Make function, the Write Sequential function writes the 128-byte data
    // record at the current DMA address to the file named by the FCB. The
    // record is placed at position cr of the file, and the cr field is
    // automatically incremented to the next record position. If the cr field
    // overflows, the next logical extent is automatically opened and the cr
    // field is reset to zero in preparation for the next write operation. Write
    // operations can take place into an existing file, in which case newly
    // written records overlay those that already exist in the file. Register A
    // = 00H upon return from a successful write operation, while a nonzero
    // value indicates an unsuccessful write caused by a full disk.
    env.save_buffer();
    let mut fcb = Fcb::new(fcb_address, env.machine);
    let record = fcb.get_sequential_record_number();
    if env.call_trace {
        print!("[Write record {:x} from {:04x}]", record, env.state.dma);
    }
    fcb.inc_current_record();
    write_record_from_buffer(&env.state.buffer, &fcb, record as u16).unwrap_or(1)
}

pub fn read_rand(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Read Random function is similar to the sequential file read operation
    // of previous releases, except that the read operation takes place at a
    // particular record number, selected by the 24-bit value constructed from
    // the 3-byte field following the FCB (byte positions r0 at 33, r1 at 34,
    // and r2 at 35). The user should note that the sequence of 24 bits is
    // stored with least significant byte first (r0), middle byte next (r1), and
    // high byte last (r2). CP/M does not reference byte r2, except in computing
    // the size of a file (see Function 35). Byte r2 must be zero, however,
    // since a nonzero value indicates overflow past the end of file.
    // Thus, the r0, r1 byte pair is treated as a double-byte, or word value,
    // that contains the record to read. This value ranges from 0 to 65535,
    // providing access to any particular record of the 8-megabyte file. To
    // process a file using random access, the base extent (extent 0) must first
    // be opened. Although the base extent might or might not contain any
    // allocated data, this ensures that the file is properly recorded in the
    // directory and is visible in DIR requests. The selected record number is
    // then stored in the random record field (r0, r1), and the BDOS is called
    // to read the record.
    // Upon return from the call, register A either contains an error ºcode, as
    // listed below, or the value 00, indicating the operation was successful.
    // In the latter case, the current DMA address contains the randomly
    // accessed record. Note that contrary to the sequential read operation,
    // the record number is not advanced. Thus, subsequent random read
    // operations continue to read the same record.
    // Upon each random read operation, the logical extent and current record
    // values are automatically set. Thus, the file can be sequentially read or
    // written, starting from the current randomly accessed position. However,
    // note that, in this case, the last randomly read record will be reread as
    // one switches from random mode to sequential read and the last record will
    // be rewritten as one switches to a sequential write operation. The user
    // can simply advance the random record position following each random read
    // or write to obtain the effect of sequential I/O operation.
    // Error codes returned in register A following a random read are listed
    // below:
    //    01	reading unwritten data
    //    02	(not returned in random mode)
    //    03	cannot close current extent
    //    04	seek to unwritten extent
    //    05	(not returned in read mode)
    //    06	seek Past Physical end of disk
    // Error codes 01 and 04 occur when a random read operation accesses a data
    // block that has not been previously written or an extent that has not been
    // created, which are equivalent conditions. Error code 03 does not normally
    // occur under proper system operation. If it does, it can be cleared by
    // simply rereading or reopening extent zero as long as the disk is not
    // physically write protected. Error code 06 occurs whenever byte r2 is
    // nonzero under the current 2.0 release. Normally, nonzero return codes can
    // be treated as missing data, with zero return codes indicating operation
    // complete.
    let fcb = Fcb::new(fcb_address, env.machine);
    let record = fcb.get_random_record_number();
    if env.call_trace {
        print!("[Read random record {:x} into {:04x}]", record, env.state.dma);
    }
    if record > 65535 {
        return 6; //06	seek Past Physical end of disk
    }
    let res = read_record_in_buffer(&mut env.state.buffer, &fcb, record as u16).unwrap_or(1);
    if res == DIRECTORY_CODE {
        env.load_buffer();
    }
    res
}

fn read_record_in_buffer(buffer: &mut[u8], fcb: &Fcb, record: u16) -> io::Result<u8> {
    let paths = find_host_files(fcb.get_name(), false)?;
    let mut os_file = fs::File::open(&paths[0])?;

    let file_offset = record as u64 * RECORD_SIZE as u64;
    if file_offset >= os_file.metadata()?.len() {
        return Ok(1); // End of file
    }

    os_file.seek(io::SeekFrom::Start(file_offset))?;
    let size = os_file.read(buffer)?;

    // Fill with zeros
    for i in size..RECORD_SIZE {
        buffer[i] = 26; // (CTRL-Z) 
    }
    Ok(0)
}

fn write_record_from_buffer(buffer: &[u8], fcb: &Fcb, record: u16) -> io::Result<u8> {
    let paths = find_host_files(fcb.get_name(), false)?;
    let mut os_file = fs::OpenOptions::new().write(true).open(&paths[0])?;

    let file_offset = record as u64 * RECORD_SIZE as u64;
    os_file.seek(io::SeekFrom::Start(file_offset))?;
    let size = os_file.write(buffer)?;

    if size != RECORD_SIZE {
        return Err(io::Error::new(io::ErrorKind::Other, "Record not fully written"));
    }    
    Ok(0)
}

pub fn get_set_user_number(env: &mut BdosEnvironment, user: u8) -> u8 {
    /*
    An application program can change or interrogate the currently
    active user number by calling Function 32. If register E = 0FFH,
    the value of the current user number is returned in register A,
    where the value is in the range of 0 to 15. If register E is
    not 0FFH, the current user number is changed to the value of E,
    modulo 16.
    */
    if user != 0xff {
        env.set_user(user & 0x0f);
    }
    env.user()
}

pub fn search_first(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // Search First scans the directory for a match with the file given by the
    // FCB addressed by DE. The value 255 (hexadecimal FF) is returned if the
    // file is not found; otherwise, 0, 1, 2, or 3 is returned indicating the
    // file is present. When the file is found, the current DMA address is
    // filled with the record containing the directory entry, and the relative
    // starting position is A * 32 (that is, rotate the A register left 5 bits,
    // or ADD A five times). Although not normally required for application
    // programs, the directory information can be extracted from the buffer at
    // this position.
    // An ASCII question mark (63 decimal, 3F hexadecimal) in any position from
    // f1 through ex matches the corresponding field of any directory entry on
    // the default or auto-selected disk drive. If the dr field contains an
    // ASCII question mark, the auto disk select function is disabled and the
    // default disk is searched, with the search function returning any matched
    // entry, allocated or free, belonging to any user number. This latter
    // function is not normally used by application programs, but it allows
    // complete flexibility to scan all current directory values. If the dr
    // field is not a question mark, the s2 byte is automatically zeroed. 
    let fcb = Fcb::new(fcb_address, env.machine);
    if env.call_trace {
        print!("[[DIR start {}]]", fcb.get_name());
    }
    env.state.dir_pattern = fcb.get_name();
    env.state.dir_pos = 0;
    search_nth(env)
}

pub fn search_next(env: &mut BdosEnvironment) -> u8 {
    // The Search Next function is similar to the Search First function, except
    // that the directory scan continues from the last matched entry. Similar to
    // Function 17, Function 18 returns the decimal value 255 in A when no more
    // directory items match. 
    search_nth(env)
}

fn search_nth(env: &mut BdosEnvironment) -> u8 {
    // For search_first and search_next, I will store a global index for the
    // position. I don't know if BDOS was storing the state on the FCB or
    // globally.
    let mut i = 0 as u16;
    match fs::read_dir("./") {
        Err(_) => FILE_NOT_FOUND,
        Ok(dir) => {
            for entry in dir {
                if let Ok(file) = entry {
                    if let Ok(file_type) = file.file_type() {
                        if file_type.is_file() {
                            let os_name = file.file_name();
                            if let Some(cpm_name) = name_to_8_3(&os_name.to_string_lossy()) {
                                if name_match(&cpm_name, &env.state.dir_pattern) {
                                    // Fits the pattern
                                    if i == env.state.dir_pos {
                                        // This is the one to show
                                        build_directory_entry(&mut env.state.buffer, cpm_name);
                                        env.state.dir_pos += 1;
                                        env.load_buffer();
                                        return DIRECTORY_CODE;
                                    }
                                    i += 1;
                                }
                            }
                        }
                    }
                }
            }
            FILE_NOT_FOUND // No more items
        }
    }
}

fn build_directory_entry(buffer: &mut [u8], cpm_name: String) {
    // Some commands return a directory record. It can hold 4 directoy entries,
    // but we only use the first one.

    // Zero the buffer
    for i in 0..RECORD_SIZE {
        buffer[i] = 0;
    }

    // Store name in the first entry
    let bytes = cpm_name.as_bytes();
    for i in 0..8 {
        buffer[1+i] = 0x7F & bytes[i as usize];
    }
    for i in 0..3 {
        buffer[9+i] = 0x7F & bytes[9 + i as usize];
    }

    // This user-number byte serves a second purpose. If this byte is set to a
    // value of 0E5H, CP/M considers that the file directory entry has been
    // deleted and completely ignores the remaining 31 bytes of data.
    buffer[32] = 0xe5;
    buffer[64] = 0xe5;
    buffer[96] = 0xe5;
}

fn find_host_files(name: String, wildcard: bool) -> io::Result<Vec<OsString>> {
    let dir = fs::read_dir("./")?;
    let mut files = Vec::new();
    for entry in dir {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let cpm_name = name_to_8_3(&entry.file_name().to_string_lossy());
            if let Some(cpm_name) = cpm_name {
                if cpm_name == name || (wildcard && name_match(&cpm_name, &name)) {
                    // File found
                    files.push(entry.path().into_os_string());
                }
            }
        }
    }
    if files.len() == 0 {
        Err(io::Error::new(io::ErrorKind::NotFound, ""))
    } else {
        Ok(files)
    }
}

// An ambiguous file reference is used for directory search and pattern
// matching. The form of an ambiguous file reference is similar to an
// unambiguous reference, except the symbol ? can be interspersed throughout the
// primary and secondary names. In various commands throughout CP/M, the ?
// symbol matches any character of a filename in the ? position. Thus, the
// ambiguous reference "X?Z.C?M" matches the following unambiguous filenames
// "XYZ.COM" and "X3Z.CAM". The wildcard character can also be used in an
// ambiguous file reference. The * character replaces all or part of a filename
// or filetype. Note that "*.*" equals the ambiguous file reference
// "????????.???" while "filename.*" and "*.typ" are abbreviations for
// "filename.???" and "????????.typ" respectively.
const WILDCARD: u8 = '?' as u8;
pub fn name_match(name: &str, pattern: &str) -> bool {
    let n = name.as_bytes();
    let p = pattern.as_bytes();
    for i in 0..(8+1+3) {
        if (n[i] != p[i]) && (p[i] != WILDCARD) {
            return false;
        }
    }
    true
}
