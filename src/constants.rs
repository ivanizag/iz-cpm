
pub const FCB1_ADDRESS:          u16 = 0x005c;
pub const FCB2_ADDRESS:          u16 = 0x006c;
pub const SYSTEM_PARAMS_ADDRESS: u16 = 0x0080; // Also default DMA buffer
pub const TPA_BASE_ADDRESS:      u16 = 0x0100;
pub const CCP_BASE_ADDRESS:      u16 = 0xf000; // The CCP binary has to be rebuilt if this changes
pub const TPA_STACK_ADDRESS:     u16 = 0xf080; // 16 bytes for an 8 level stack
pub const BDOS_BASE_ADDRESS:     u16 = 0xf800;
pub const BIOS_BASE_ADDRESS:     u16 = 0xff00;
