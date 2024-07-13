use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::bdos_environment::*;
use crate::constants::*;
use crate::fcb::*;
use iz80::Machine;

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
const NO_DATA: u8 = 1;
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
    let mut fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Open file {}]]", fcb.get_name_for_log(env));
    }
    match find_host_files(env, &fcb, false, false) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(paths) => match fs::File::open(&paths[0]) {
            Err(_) => FILE_NOT_FOUND,
            Ok(os_file) => match os_file_size_records(os_file) {
                Err(_) => FILE_NOT_FOUND,
                Ok(record_count) => {
                    fcb.init(env, record_count);
                    DIRECTORY_CODE
                }
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
    let mut fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Create file {}]]", fcb.get_name_for_log(env));
    }
    match create_file(env, &fcb) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(_) => {
            fcb.init(env, 0);
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
    let fcb = Fcb::new(fcb_address);
    match find_host_files(env, &fcb, false, false){
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(paths) => match truncate_if_needed(env, &fcb, &paths[0]) {
            Err(_) => FILE_NOT_FOUND,
            Ok(_) => DIRECTORY_CODE
        }
    }
}

fn truncate_if_needed(env: &mut BdosEnvironment, fcb: &Fcb, os_file_name: &OsString) -> io::Result<()> {
    let os_file = fs::File::open(os_file_name)?;
    let record_count = os_file_size_records(os_file)?;
    let (extent_is_full, fcb_record_count) = fcb.get_record_count(env);
    if extent_is_full {
        return Ok(()); // No truncation needed and it could not be the last extent.
    }
    if record_count == fcb_record_count as u32 {
        return Ok(());
    }

    if env.call_trace {
        println!("Truncating file from {} to {}", record_count, fcb_record_count);
    }

    let file = fs::OpenOptions::new().write(true).open(os_file_name)?;
    file.set_len(fcb_record_count as u64 * RECORD_SIZE as u64)?;
    return Ok(());
}

pub fn delete(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Delete File function removes files that match the FCB addressed by
    // DE. The filename and type may contain ambiguous references (that is,
    // question marks in various positions), but the drive select code cannot be
    // ambiguous, as in the Search and Search Next functions.
    // Function 19 returns a decimal 255 if the referenced file or files cannot
    // be found; otherwise, a value in the range 0 to 3 returned.
    let fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Delete file {}]]", fcb.get_name_for_log(env));
    }

    match find_host_files(env, &fcb, true, true) {
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

pub fn set_attributes(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Set File Attributes function allows programmatic manipulation of
    // permanent indicators attached to files. In particular, the R/O and System
    // attributes (t1' and t2') can be set or reset. The DE pair addresses an
    // unambiguous filename with the appropriate attributes set or reset.
    // Function 30 searches for a match and changes the matched directory entry
    // to contain the selected indicators. Indicators f1' through f4' are not
    // currently used, but may be useful for applications programs, since they
    // are not involved in the matching process during file open and close
    // operations. Indicators f5' through f8' and t3' are reserved for future
    // system expansion.
    let fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Set attribuyes {}]]", fcb.get_name_for_log(env));
    }

    match find_host_files(env, &fcb, false, true) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(_) => DIRECTORY_CODE // TODO: Do something with this.
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
    let fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Rename file {} to {}]]", fcb.get_name_for_log(env), fcb.get_name_secondary(env));
    }
    match find_host_files(env, &fcb, false, true) {
        Err(_) => FILE_NOT_FOUND, // Error or file not found
        Ok(paths) => {
            for name in paths {
                let src_path = Path::new(&name);
                let new_name = name_from_8_3(&fcb.get_name_secondary(env));
                let mut dst_path = PathBuf::from(src_path.parent().unwrap_or_else(|| Path::new("")));
                dst_path.push(new_name);

                if fs::rename(src_path, dst_path).is_err() {
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
    let mut fcb = Fcb::new(fcb_address);
    let record = fcb.get_sequential_record_number(env);
    if env.call_trace {
        print!("[Read record {:x} into {:04x}]", record, env.state.dma);
    }

    let extent_changed = fcb.inc_current_record(env);

    let mut buffer: Buffer = [0; RECORD_SIZE]; 
    let res = read_record_in_buffer(env, &fcb, record as u16, &mut buffer).unwrap_or(NO_DATA);
    if res == DIRECTORY_CODE {
        env.store_buffer_to_dma(&buffer);
    }

    if extent_changed {
        match update_record_count(env, &mut fcb) {
            Err(_) => return NO_DATA,
            Ok(_) => {}
        }
    }
    res
}

fn update_record_count(env: &mut BdosEnvironment, fcb: &mut Fcb) -> io::Result<()> {
    let record_count = compute_file_size_internal(env, &fcb)?;
    fcb.update_record_count(env, record_count);
    Ok(())
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
    let mut fcb = Fcb::new(fcb_address);
    let record = fcb.get_sequential_record_number(env);
    if env.call_trace {
        print!("[Write record {:x} from {:04x}]", record, env.state.dma);
    }

    let buffer = env.load_buffer_from_dma();
    let result = write_record_from_buffer(env, &fcb, record as u16, &buffer).unwrap_or(NO_DATA);

    fcb.inc_current_record(env);
    match update_record_count(env, &mut fcb) {
        Err(_) => return NO_DATA,
        Ok(_) => {}
    }

    result
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
    // Upon return from the call, register A either contains an error code, as
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
    let fcb = Fcb::new(fcb_address);
    let record = fcb.get_random_record_number(env);
    if env.call_trace {
        print!("[Read random record {:x} into {:04x}]", record, env.state.dma);
    }
    if record > 65535 {
        return 6; //06	seek Past Physical end of disk
    }
    let mut buffer: Buffer = [0; RECORD_SIZE];
    let res = read_record_in_buffer(env, &fcb, record as u16, &mut buffer).unwrap_or(NO_DATA);
    if res == DIRECTORY_CODE {
        env.store_buffer_to_dma(&buffer);
    }
    res
}

pub fn write_rand(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Write Random operation is initiated similarly to the Read Random
    // call, except that data is written to the disk from the current DMA
    // address. Further, if the disk extent or data block that is the target of
    // the write has not yet been allocated, the allocation is performed before
    // the write operation continues. As in the Read Random operation, the
    // random record number is not changed as a result of the write. The logical
    // extent number and current record positions of the FCB are set to
    // correspond to the random record that is being written. Again, sequential
    // read or write operations can begin following a random write, with the
    // notation that the currently addressed record is either read or rewritten
    // again as the sequential operation begins. You can also simply advance the
    // random record position following each write to get the effect of a
    // sequential write operation. Note that reading or writing the last record
    // of an extent in random mode does not cause an automatic extent switch as
    // it does in sequential mode.
    // The error codes returned by a random write are identical to the random
    // read operation with the addition of error code 05, which indicates that a
    // new extent cannot be created as a result of directory overflow.
    let fcb = Fcb::new(fcb_address);
    let record = fcb.get_random_record_number(env);
    if env.call_trace {
        print!("[Write random record {:x} into {:04x}]", record, env.state.dma);
    }
    if record > 65535 {
        return 6; //06	seek Past Physical end of disk
    }

    let buffer = env.load_buffer_from_dma();
    write_record_from_buffer(env, &fcb, record as u16, &buffer).unwrap_or(NO_DATA)
}

pub fn write_rand_zero_fill(env: &mut BdosEnvironment, fcb_address: u16) -> u8 {
    // The Write With Zero Fill operation is similar to Function 34, with the
    // exception that a previously unallocated block is filled with zeros before
    // the data is written.

    // On this emulator, spares files are managed by the host operating systems
    // if possible. So, this method is exactly the same as function 34.
    write_rand(env, fcb_address)
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
        env.state.user = user & 0x0f;

        // Update the RAM byte to mark our drive/user in a persistent way.
        env.machine.poke(CCP_USER_DRIVE_ADDRESS, env.state.user << 4 | env.state.drive)

    }

    env.state.user
}

pub fn set_error_mode(_env: &mut BdosEnvironment, _mode: u8) {
    /*
    Instructs CP/M what action to take if there is a hardware error:
    E < 254
        Compatibility mode; program is terminated and an error message printed.
    E = 254
        Error code is returned in H, error message is printed.
    E = 255
        Error code is returned in H, no error message is printed.
    Note that the messages (if printed) are not followed by a carriage return or linefeed.

    Not in CP/M 2.2 per the "CP/M Operating System Manual", but
    the BBC BASIC version for Z80 uses it.

    */
    // Wo don't do anything with this on the emulator.
}


pub fn set_random_record(env: &mut BdosEnvironment, fcb_address: u16) {
    // The Set Random Record function causes the BDOS automatically to produce
    // the random record position from a file that has been read or written
    // sequentially to a particular point. The function can be useful in two
    // ways.
    // First, it is often necessary initially to read and scan a sequential file
    // to extract the positions of various key fields. As each key is
    // encountered, Function 36 is called to compute the random record position
    // for the data corresponding to this key. If the data unit size is 128
    // bytes, the resulting record position is placed into a table with the key
    // for later retrieval. After scanning the entire file and tabulating the
    // keys and their record numbers, the user can move instantly to a
    // particular keyed record by performing a random read, using the
    // corresponding random record number that was saved earlier. The scheme is
    // easily generalized for variable record lengths, because the program need
    // only store the buffer-relative byte position along with the key and
    // record number to find the exact starting position of the keyed data at a
    // later time.
    // A second use of Function 36 occurs when switching from a sequential read
    // or write over to random read or write. A file is sequentially accessed to
    // a particular point in the file, Function 36 is called, which sets the
    // record number, and subsequent random read and write operations continue
    // from the selected point in the file.
    let mut fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Set pos of {}]]", fcb.get_name_for_log(env));
    }
    let record = fcb.get_sequential_record_number(env);
    fcb.set_random_record_number(env, record as u32);
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
    let fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[DIR start {}]]", fcb.get_name_for_log(env));
    }
    env.state.dir_drive = fcb.get_drive(env);
    env.state.dir_pattern = fcb.get_name(env);
    env.state.dir_pos = 0;
    search_nth(env).unwrap_or(FILE_NOT_FOUND)
}

pub fn search_next(env: &mut BdosEnvironment) -> u8 {
    // The Search Next function is similar to the Search First function, except
    // that the directory scan continues from the last matched entry. Similar to
    // Function 17, Function 18 returns the decimal value 255 in A when no more
    // directory items match.
    search_nth(env).unwrap_or(FILE_NOT_FOUND)
}

pub fn compute_file_size(env: &mut BdosEnvironment, fcb_address: u16) {
    // When computing the size of a file, the DE register pair addresses an FCB
    // in random mode format (bytes r0, r1, and r2 are present). The FCB
    // contains an unambiguous filename that is used in the directory scan. Upon
    // return, the random record bytes contain the virtual file size, which is,
    // in effect, the record address of the record following the end of the
    // file. Following a call to Function 35, if the high record byte r2 is 01,
    // the file contains the maximum record count 65536. Otherwise, bytes r0 and
    // r1 constitute a 16-bit value as before (r0 is the least significant
    // byte), which is the file size.
    // Data can be appended to the end of an existing file by simply calling
    // Function 35 to set the random record position to the end-of-file and then
    // performing a sequence of random writes starting at the preset record
    // address.
    // The virtual size of a file corresponds to the physical size when the file
    // is written sequentially. If the file was created in random mode and holes
    // exist in the allocation, the file might contain fewer records than the
    // size indicates. For example, if only the last record of an 8-megabyte
    // file is written in random mode (that is, record number 65535), the
    // virtual size is 65536 records, although only one block of data is
    // actually allocated.
    let mut fcb = Fcb::new(fcb_address);
    if env.call_trace {
        print!("[[Size of {}]]", fcb.get_name_for_log(env));
    }
    let size = compute_file_size_internal(env, &fcb);
    match size {
        Err(_) => (),
        Ok(size) => {
            fcb.set_random_record_number(env, size);
        }
    }
}

fn compute_file_size_internal(env: &mut BdosEnvironment, fcb: &Fcb) -> io::Result<u32> {
    let paths = find_host_files(env, fcb, false, false)?;
    let os_file = fs::File::open(&paths[0])?;
    os_file_size_records(os_file)
}

fn os_file_size_records(os_file: fs::File) -> io::Result<u32> {
    let file_size = os_file.metadata()?.len();
    let mut record = file_size / RECORD_SIZE as u64;
    if file_size % RECORD_SIZE as u64 != 0 {
        // We need integer division rounding up.
        record += 1;
    }

    if record >= 65536 {
        record = 65536;
    }
    Ok(record as u32)
}

fn find_host_files(env: &mut BdosEnvironment, fcb: &Fcb, wildcard: bool, to_write: bool) -> io::Result<Vec<OsString>> {
    let fcb_drive = fcb.get_drive(env);
    let path = env.get_directory(fcb_drive, to_write)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No directory assigned to drive"))?;
    let dir = fs::read_dir(path)?;
    let mut files = Vec::new();
    for entry in dir {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let fcb_name = fcb.get_name(env);
            let cpm_name = name_to_8_3(&entry.file_name().to_string_lossy());
            if let Some(cpm_name) = cpm_name {
                if cpm_name == fcb_name || (wildcard && name_match(&cpm_name, &fcb_name)) {
                    // File found
                    files.push(entry.path().into_os_string());
                }
            }
        }
    }
    if files.is_empty() {
        Err(io::Error::new(io::ErrorKind::NotFound, "Empty drive"))
    } else {
        Ok(files)
    }
}

fn create_file(env: &mut BdosEnvironment, fcb: &Fcb) -> io::Result<()> {
    let fcb_drive = fcb.get_drive(env);
    let path = env.get_directory(fcb_drive, true)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No directory assigned to drive"))?;
    let file = Path::new(&path).join(fcb.get_name_host(env));
    fs::File::create(&file)?;
    Ok(())
}

fn read_record_in_buffer(env: &mut BdosEnvironment, fcb: &Fcb, record: u16, buffer: &mut Buffer) -> io::Result<u8> {
    let paths = find_host_files(env, fcb, false, false)?;
    let mut os_file = fs::File::open(&paths[0])?;

    let file_offset = record as u64 * RECORD_SIZE as u64;
    if file_offset >= os_file.metadata()?.len() {
        return Ok(1); // End of file
    }

    os_file.seek(io::SeekFrom::Start(file_offset))?;
    let size = os_file.read(buffer)?;

    // Fill with ctrl-Z
    for i in size..RECORD_SIZE {
        buffer[i] = 26; // (CTRL-Z)
    }
    Ok(0)
}

fn write_record_from_buffer(env: &mut BdosEnvironment, fcb: &Fcb, record: u16, buffer: &[u8]) -> io::Result<u8> {
    let paths = find_host_files(env, fcb, false, true)?;
    let mut os_file = fs::OpenOptions::new().write(true).open(&paths[0])?;

    let file_offset = record as u64 * RECORD_SIZE as u64;
    let file_pos = os_file.seek(io::SeekFrom::Start(file_offset))?;

    if file_offset > file_pos {
        // We want to write past the end of the file. Seek wasn't able to get
        // there, so we will complete the holes with zeros as needed.
        let zero = [0_u8];
        let needed = file_offset - file_pos;
        for _ in 0..needed {
            os_file.write_all(&zero)?;
        }
    }

    os_file.write_all(buffer)?;
    Ok(0)
}

fn search_nth(env: &mut BdosEnvironment) -> io::Result<u8> {
    // For search_first and search_next, I will store a global index for the
    // position. I don't know if BDOS was storing the state on the FCB or
    // globally. [Later] Yes, it does.
    let path = env.get_directory(env.state.dir_drive, false)
        .ok_or(io::Error::new(io::ErrorKind::Other, "No directory assigned to drive"))?;
    let dir = fs::read_dir(path)?;

    let mut i = 0;
    for entry in dir {
        let file = entry?;
        if file.file_type()?.is_file() {
            let os_name = file.file_name();
            if let Some(cpm_name) = name_to_8_3(&os_name.to_string_lossy()) {
                if name_match(&cpm_name, &env.state.dir_pattern) {
                    // Fits the pattern
                    if i == env.state.dir_pos {
                        // This is the one to show
                        let buffer = build_directory_entry(cpm_name);
                        env.state.dir_pos += 1;
                        env.store_buffer_to_dma(&buffer);
                        return Ok(DIRECTORY_CODE);
                    }
                    i += 1;
                }
            }
        }
    }
    Ok(FILE_NOT_FOUND) // No more items
}

fn build_directory_entry(cpm_name: String) -> Buffer {
    // Some commands return a directory record. It can hold 4 directoy entries,
    // but we only use the first one.

    // Zero the buffer
    let mut buffer = [0; RECORD_SIZE];

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

    buffer
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
const WILDCARD: u8 = b'?';
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