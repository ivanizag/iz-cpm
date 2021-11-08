// Zero page
pub const IOBYTE_ADDRESS:         u16 = 0x0003;
pub const CCP_USER_DRIVE_ADDRESS: u16 = 0x0004; // Updated by CCP, not BDOS
pub const BDOS_ENTRY_ADDRESS:     u16 = 0x0005;
pub const FCB1_ADDRESS:           u16 = 0x005c;
pub const FCB2_ADDRESS:           u16 = 0x006c;
pub const SYSTEM_PARAMS_ADDRESS:  u16 = 0x0080; // Also default DMA buffer

// Memory map
pub const TPA_BASE_ADDRESS:       u16 = 0x0100;
pub const CCP_BASE_ADDRESS:       u16 = 0xf000; // The CCP binary has to be rebuilt if this changes (third-party/build_zcpr.sh)
pub const TPA_STACK_ADDRESS:      u16 = 0xf080; // 16 bytes for an 8 level stack
pub const BDOS_BASE_ADDRESS:      u16 = 0xf800;
pub const BDOS_DPB0_ADDRESS:      u16 = 0xf900;
pub const BDOS_ALVEC0_ADDRESS:    u16 = 0xf910;
pub const BIOS_BASE_ADDRESS:      u16 = 0xff00;

// Exit conditions
#[derive(PartialEq)]
pub enum ExecutionResult {
    Continue,
    WarmBoot,
    ColdBoot,
    Stop,
    StopConfirm,
}