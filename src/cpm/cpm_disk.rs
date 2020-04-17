static DEFAULT_DMA: u16 = 0xff00;

pub struct CpmDisk {
    selected: u8,
    dma: u16
}

impl CpmDisk {
    pub fn new() -> CpmDisk {
        CpmDisk {
            selected: 0,
            dma: DEFAULT_DMA
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
        self.selected = 0;
    }

    pub fn select(&mut self, selected: u8) {
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
        self.selected = selected;
    }
}