#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Read Control Register"]
    pub readctrl: READCTRL,
    #[doc = "0x08 - Write Control Register"]
    pub writectrl: WRITECTRL,
    #[doc = "0x0c - Write Command Register"]
    pub writecmd: WRITECMD,
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    pub addrb: ADDRB,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Write Data Register"]
    pub wdata: WDATA,
    #[doc = "0x1c - Status Register"]
    pub status: STATUS,
    _reserved7: [u8; 0x10],
    #[doc = "0x30 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x34 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x40 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x44 - Flash Cache Command Register"]
    pub cachecmd: CACHECMD,
    #[doc = "0x48 - Cache Hits Performance Counter"]
    pub cachehits: CACHEHITS,
    #[doc = "0x4c - Cache Misses Performance Counter"]
    pub cachemisses: CACHEMISSES,
    _reserved15: [u8; 0x04],
    #[doc = "0x54 - Mass Erase Lock Register"]
    pub masslock: MASSLOCK,
    #[doc = "0x58 - Irq Latency Register"]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x5c - Startup Control"]
    pub startup: STARTUP,
    _reserved18: [u8; 0x14],
    #[doc = "0x74 - Command Register"]
    pub cmd: CMD,
    _reserved19: [u8; 0x18],
    #[doc = "0x90 - Bootloader Read and Write Enable, Write Once Register"]
    pub bootloaderctrl: BOOTLOADERCTRL,
    #[doc = "0x94 - Software Unlock AAP Command Register"]
    pub aapunlockcmd: AAPUNLOCKCMD,
    #[doc = "0x98 - Cache Configuration Register 0"]
    pub cacheconfig0: CACHECONFIG0,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "READCTRL (rw) register accessor: an alias for `Reg<READCTRL_SPEC>`"]
pub type READCTRL = crate::Reg<readctrl::READCTRL_SPEC>;
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "WRITECTRL (rw) register accessor: an alias for `Reg<WRITECTRL_SPEC>`"]
pub type WRITECTRL = crate::Reg<writectrl::WRITECTRL_SPEC>;
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "WRITECMD (w) register accessor: an alias for `Reg<WRITECMD_SPEC>`"]
pub type WRITECMD = crate::Reg<writecmd::WRITECMD_SPEC>;
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "ADDRB (rw) register accessor: an alias for `Reg<ADDRB_SPEC>`"]
pub type ADDRB = crate::Reg<addrb::ADDRB_SPEC>;
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "WDATA (rw) register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "Write Data Register"]
pub mod wdata;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CACHECMD (w) register accessor: an alias for `Reg<CACHECMD_SPEC>`"]
pub type CACHECMD = crate::Reg<cachecmd::CACHECMD_SPEC>;
#[doc = "Flash Cache Command Register"]
pub mod cachecmd;
#[doc = "CACHEHITS (r) register accessor: an alias for `Reg<CACHEHITS_SPEC>`"]
pub type CACHEHITS = crate::Reg<cachehits::CACHEHITS_SPEC>;
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "CACHEMISSES (r) register accessor: an alias for `Reg<CACHEMISSES_SPEC>`"]
pub type CACHEMISSES = crate::Reg<cachemisses::CACHEMISSES_SPEC>;
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "MASSLOCK (rw) register accessor: an alias for `Reg<MASSLOCK_SPEC>`"]
pub type MASSLOCK = crate::Reg<masslock::MASSLOCK_SPEC>;
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
#[doc = "IRQLATENCY (rw) register accessor: an alias for `Reg<IRQLATENCY_SPEC>`"]
pub type IRQLATENCY = crate::Reg<irqlatency::IRQLATENCY_SPEC>;
#[doc = "Irq Latency Register"]
pub mod irqlatency;
#[doc = "STARTUP (rw) register accessor: an alias for `Reg<STARTUP_SPEC>`"]
pub type STARTUP = crate::Reg<startup::STARTUP_SPEC>;
#[doc = "Startup Control"]
pub mod startup;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "BOOTLOADERCTRL (rw) register accessor: an alias for `Reg<BOOTLOADERCTRL_SPEC>`"]
pub type BOOTLOADERCTRL = crate::Reg<bootloaderctrl::BOOTLOADERCTRL_SPEC>;
#[doc = "Bootloader Read and Write Enable, Write Once Register"]
pub mod bootloaderctrl;
#[doc = "AAPUNLOCKCMD (w) register accessor: an alias for `Reg<AAPUNLOCKCMD_SPEC>`"]
pub type AAPUNLOCKCMD = crate::Reg<aapunlockcmd::AAPUNLOCKCMD_SPEC>;
#[doc = "Software Unlock AAP Command Register"]
pub mod aapunlockcmd;
#[doc = "CACHECONFIG0 (rw) register accessor: an alias for `Reg<CACHECONFIG0_SPEC>`"]
pub type CACHECONFIG0 = crate::Reg<cacheconfig0::CACHECONFIG0_SPEC>;
#[doc = "Cache Configuration Register 0"]
pub mod cacheconfig0;
