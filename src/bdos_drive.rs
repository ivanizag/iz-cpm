use super::bdos_environment::*;

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
    let drive = selected & 0x0f;
    env.set_drive(drive);
    env.state.selected_bitmap &= 1 << drive;
}

pub fn get_current(env: &mut BdosEnvironment) -> u8 {
    // Function 25 returns the currently selected default disk number in
    // register A. The disk numbers range from 0 through 15 corresponding to
    // drives A through P.
    env.drive()
}

pub fn get_log_in_vector(env: &mut BdosEnvironment) -> u16 {
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