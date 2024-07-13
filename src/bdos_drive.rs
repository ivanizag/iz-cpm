use std::fs;
use std::io;

use crate::bdos_environment::*;
use crate::constants::*;
use crate::fcb::name_to_8_3;
use iz80::Machine;

pub fn all_reset(env: &mut BdosEnvironment) -> u8{
    // The Reset Disk function is used to programmatically restore the file
    // system to a reset state where all disks are set to Read-Write. Only
    // disk drive A is selected, and the default DMA address is reset to
    // BOOT+0080H. This function can be used, for example, by an application
    // program that requires a disk change without a system reboot.
    // In versions 1 and 2, logs in drive A: and returns 0FFh if there is a file
    // present whose name begins with a $, otherwise 0. Replacement BDOSses may
    // modify this behaviour.
    env.state.reset();

    match has_dollar_file(env) {
        Ok(true) => 0xff,
        _ => 0,
    }
}

fn has_dollar_file(env: &mut BdosEnvironment) -> io::Result<bool> {
    let path = env.get_directory(0, false)
        .ok_or(io::Error::new(io::ErrorKind::Other, "No directory assigned to drive"))?;
    let dir = fs::read_dir(path)?;

    for entry in dir {
        let file = entry?;
        if file.file_type()?.is_file() {
            let os_name = file.file_name();
            if let Some(cpm_name) = name_to_8_3(&os_name.to_string_lossy()) {
                if cpm_name.starts_with("$") {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

pub fn select(env: &mut BdosEnvironment, selected: u8) {
    // The Select Disk function designates the disk drive named in register E as
    // the default disk for subsequent file operations, with E = 0 for drive A,
    // 1 for drive B, and so on through 15, corresponding to drive P in a full
    // 16 drive system. The drive is placed in an on-line status, which
    // activates its directory until the next cold start, warm start, or disk
    // system reset operation. If the disk medium is changed while it is
    // on-line, the drive automatically goes to a Read-Only status in a standard
    // CP/M environment, see Function 28. FCBs that specify drive code zero (dr
    // = 00H) automatically reference the currently selected default drive.
    // Drive code values between 1 and 16 ignore the selected default drive and
    // directly reference drives A through P.
    env.state.drive = selected & 0x0f;
    env.state.selected_bitmap |= 1 << env.state.drive;


    // Update the RAM byte to mark our drive/user in a persistent way.
    env.machine.poke(CCP_USER_DRIVE_ADDRESS, env.state.user << 4 | env.state.drive)
}

pub fn get_current(env: &BdosEnvironment) -> u8 {
    // Function 25 returns the currently selected default disk number in
    // register A. The disk numbers range from 0 through 15 corresponding to
    // drives A through P.
    env.state.drive
}

pub fn get_log_in_vector(env: &BdosEnvironment) -> u16 {
    // The log-in vector value returned by CP/M is a 16-bit value in HL, where
    // the least significant bit of L corresponds to the first drive A and the
    // high-order bit of H corresponds to the sixteenth drive, labeled P. A 0
    // bit indicates that the drive is not on-line, while a 1 bit marks a drive
    // that is actively on-line as a result of an explicit disk drive selection
    // or an implicit drive select caused by a file operation that specified a
    // nonzero dr field. The user should note that compatibility is maintained
    // with earlier releases, because registers A and L contain the same values
    // upon return.
    env.state.selected_bitmap
}

pub fn set_disk_read_only(env: &mut BdosEnvironment) {
    // The Write Protect Disk function provides temporary write protection for
    // the currently selected disk. Any attempt to write to the disk before the
    // next cold or warm start operation produces the message:
    //      BDOS ERR on d: R/O
    env.state.read_only_bitmap |= 1 << env.state.drive;
}

pub fn get_read_only_vector(env: &BdosEnvironment) -> u16 {
    // Function 29 returns a bit vector in register pair HL, which indicates
    // drives that have the temporary Read-Only bit set. As in Function 24, the
    // least significant bit corresponds to drive A, while the most significant
    // bit corresponds to drive P. The R/O bit is set either by an explicit call
    // to Function 28 or by the automatic software mechanisms within CP/M that
    // detect changed disks.
    env.state.read_only_bitmap
}

pub fn get_disk_allocation_vector(_env: &BdosEnvironment) -> u16 {
    // An allocation vector is maintained in main memory for each on-line disk
    // drive. Various system programs use the information provided by the
    // allocation vector to determine the amount of remaining storage (see the
    // STAT program). Function 27 returns the base address of the allocation
    // vector for the currently selected disk drive. However, the allocation
    // information might be invalid if the selected disk has been marked
    // Read-Only. Although this function is not normally used by application
    // programs, additional details of the allocation vector are found in
    // Section 6.
    BDOS_ALVEC0_ADDRESS
}

pub fn get_disk_parameter_block(_env: &BdosEnvironment) -> u16 {
    // The address of the BIOS resident disk parameter block is returned in HL
    // as a result of this function call. This address can be used for either of
    // two purposes. First, the disk parameter values can be extracted for
    // display and space computation purposes, or transient programs can
    // dynamically change the values of current disk parameters when the disk
    // environment changes, if required. Normally, application programs will not
    // require this facility.
    BDOS_DPB0_ADDRESS
}

pub fn reset_drives(env: &mut BdosEnvironment, drives: u16) -> u8 {
    // The Reset Drive function allows resetting of specified drives. The passed
    // parameter is a 16-bit vector of drives to be reset; the least significant
    // bit is drive A:.
    // To maintain compatibility with MP/M, CP/M returns a zero value.
    env.state.selected_bitmap &= !drives;
    env.state.read_only_bitmap &= !drives;

    // Select current drive
    env.state.selected_bitmap |= 1 << env.state.drive;

    0
}
