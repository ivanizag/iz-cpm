extern crate z80;

mod cpm_console;
mod cpm_machine;

use z80::cpu::Cpu;
use z80::memory_io::*;
use z80::registers::*;
use z80::state::State;

use self::cpm_console::*;
use self::cpm_machine::*;

//static PROGRAM: &'static [u8] = include_bytes!("rom/zexall.com");
static PROGRAM: &'static [u8] = include_bytes!("rom/cbasic.com");

fn main() {
    let mut machine = CpmMachine::new();
    let mut state = State::new();
    let mut cpu = Cpu::new();

    // Init console
    let mut console = CpmConsole::new();

    // Load program
    let code = PROGRAM;
    let size = code.len();
    for i in 0..size {
        machine.poke(0x100 + i as u16, code[i]);
    }

    /*
    System call 5

    .org $5
        out ($0), a
        ret
    */
    let code = [0xD3, 0x00, 0xC9];
    for i in 0..code.len() {
        machine.poke(5 + i as u16, code[i]);
    }

    state.reg.set_pc(0x100);
    let trace = false;
    cpu.trace = trace;
    loop {
        cpu.execute_instruction(&mut state, &mut machine);

        if trace {
            // CPU registers
            println!("PC({:04x}) AF({:04x}) BC({:04x}) DE({:04x}) HL({:04x}) SP({:04x}) IX({:04x}) IY({:04x}) Flags({:08b})",
                state.reg.get_pc(),
                state.reg.get16(Reg16::AF),
                state.reg.get16(Reg16::BC),
                state.reg.get16(Reg16::DE),
                state.reg.get16(Reg16::HL),
                state.reg.get16(Reg16::SP),
                state.reg.get16(Reg16::IX),
                state.reg.get16(Reg16::IY),
                state.reg.get8(Reg8::F)
            );
        }

        if state.reg.get_pc() == 0x0000 {
            println!("Terminated in address 0x0000");
            break;
        }

        // We do the BDOS actions outside the emulation just before the RTS
        if state.reg.get_pc() == 0x0007 {
            console.pool_keyboard();

            let command = state.reg.get8(Reg8::C); 
            //print!("\n[[BDOS command {}]]", command);
            match command {
                // See https://www.seasip.info/Cpm/bdos.html
                // See http://www.gaby.de/cpm/manuals/archive/cpm22htm/ch5.htm
                1=> { // C_READ) - Console input
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
                    state.reg.set_a(console.read())
                    // No need to echo. The console has done that already.
                }
                2 => { // C_WRITE - Console output
                    /*
                    The ASCII character from register E is sent to the console
                    device. As in Function 1, tabs are expanded and checks are made
                    for start/stop scroll and printer echo. 
                    */
                    console.write(state.reg.get8(Reg8::E));
                },
                9 => { // C_WRITESTR - Output string
                    /*
                    The Print String function sends the character string stored in
                    memory at the location given by DE to the console device, until
                    a $ is encountered in the string. Tabs are expanded as in
                    Function 2, and checks are made for start/stop scroll and
                    printer echo. 
                    */
                    let address = state.reg.get16(Reg16::DE);
                    console.write_string(address, &machine);
                },
                11 => { // C_STAT - Console status
                    /*
                    The Console Status function checks to see if a character has
                    been typed at the console. If a character is ready, the value
                    0FFH is returned in register A. Otherwise a 00H value is returned. 
                    */
                    state.reg.set_a(console.status());
                },
                12 => { // S_BDOSVER - Return version number
                    /*
                    Function 12 provides information that allows version independent
                    programming. A two-byte value is returned, with H = 00
                    designating the CP/M release (H = 01 for MP/M) and L = 00 for all
                    releases previous to 2.0. CP/M 2.0 returns a hexadecimal 20 in
                    register L, with subsequent version 2 releases in the hexadecimal
                    range 21, 22, through 2F. Using Function 12, for example, the
                    user can write application programs that provide both sequential
                    and random access functions. 
                    */
                    state.reg.set16(Reg16::HL, 0x22); // CPM 2.2
                },
                13 => { // DRV_ALLRESET - Reset disk system
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
                },
                14 => { // DRV_SET - Select disk
                    /*
                    The Select Disk function designates the disk drive named in
                    register E as the default disk for subsequent file operations,
                    with E = 0 for drive A, 1 for drive B, and so on through 15,
                    corresponding to drive P in a full 16 drive system. The drive is
                    placed in an on-line status, which activates its directory until
                    the next cold start, warm start, or disk system reset operation.
                    If the disk medium is changed while it is on-line, the drive
                    automatically goes to a Read-Only status in a standard CP/M
                    environment, see Function 28. FCBs that specify drive code
                    zero (dr = 00H) automatically reference the currently selected
                    default drive. Drive code values between 1 and 16 ignore the
                    selected default drive and directly reference drives A through P.
                    */
                    let selected = state.reg.get8(Reg8::E);
                    print!("[[Disk {} selected]]", selected);
                    // TODO
                    //cpm_disk.select_disk(selected);
                },
                15 => { // F_OPEN - Open file
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
                    //let res = cpm_file.open(mem, REG16::DE);
                    //state.reg.set_a(res);
                    //TODO
                    state.reg.set_a(0);
                },
                16 => { // F_CLOSE - Close file
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
                    //let res = cpm_file.close(mem, REG16::DE);
                    //state.reg.set_a(res);
                    //TODO
                    state.reg.set_a(0);
                },
                20 => { // F_READ - read next record
                    /*
                    Given that the FCB addressed by DE has been activated through an
                    Open or Make function, the Read Sequential function reads the
                    next 128-byte record from the file into memory at the current DMA
                    address. The record is read from position cr of the extent, and
                    the cr field is automatically incremented to the next record
                    position. If the cr field overflows, the next logical extent is
                    automatically opened and the cr field is reset to zero in
                    preparation for the next read operation. The value 00H is returned
                    in the A register if the read operation was successful, while a
                    nonzero value is returned if no data exist at the next record
                    position (for example, end-of-file occurs). 
                    */
                    //let res = cpm_file.read(mem, Reg16::DE);
                    //state.reg.set_a(res);
                    //TODO
                    state.reg.set_a(0xff);
                },
                24 => { // DRV_LOGINVEC - Return Log-in Vector
                    /*
                    The log-in vector value returned by CP/M is a 16-bit value in HL,
                    where the least significant bit of L corresponds to the first
                    drive A and the high-order bit of H corresponds to the sixteenth
                    drive, labeled P. A 0 bit indicates that the drive is not on-line,
                    while a 1 bit marks a drive that is actively on-line as a result of
                    an explicit disk drive selection or an implicit drive select caused
                    by a file operation that specified a nonzero dr field. The user
                    should note that compatibility is maintained with earlier releases,
                    because registers A and L contain the same values upon return. 
                    */
                    // TODO
                    // let vector = cpm_drive.log_in_vector();
                    let vector = 1;
                    state.reg.set16(Reg16::HL, vector);
                    state.reg.set_a(vector as u8);
                },
                25 => { // DRV_GET - Return current disk
                    /*
                    Function 25 returns the currently selected default disk number
                    in register A. The disk numbers range from 0 through 15
                    corresponding to drives A through P.
                    */
                    // TODO
                    // let drive = cpm_drive.current()
                    let drive = 0; // A:
                    state.reg.set_a(drive);
                },
                26 => { // F_DMAOFF - Set DMA address
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
                    //cpm_file.set_dma(state.reg.get16(REG16::DE));
                    // TODO
                },
                33 => { // F_READRAND - Random access read record
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
                    // TODO
                    //let res = cpm_file.random_read(FCBinDE);
                    let res = 1;
                    state.reg.set_a(res);
                }

                _ => {
                    print!("Command {} not implemented.\n", command);
                    panic!("BDOS command not implemented");
                }
            }
        }
    }
}