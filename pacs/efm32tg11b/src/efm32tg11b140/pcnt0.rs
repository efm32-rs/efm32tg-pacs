#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x10 - Top Value Register"]
    pub top: TOP,
    #[doc = "0x14 - Top Value Buffer Register"]
    pub topb: TOPB,
    #[doc = "0x18 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0x44 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved13: [u8; 0x1c],
    #[doc = "0x64 - Auxiliary Counter Value Register"]
    pub auxcnt: AUXCNT,
    #[doc = "0x68 - PCNT Input Register"]
    pub input: INPUT,
    #[doc = "0x6c - Oversampling Config Register"]
    pub ovscfg: OVSCFG,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "TOP (r) register accessor: an alias for `Reg<TOP_SPEC>`"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "Top Value Register"]
pub mod top;
#[doc = "TOPB (rw) register accessor: an alias for `Reg<TOPB_SPEC>`"]
pub type TOPB = crate::Reg<topb::TOPB_SPEC>;
#[doc = "Top Value Buffer Register"]
pub mod topb;
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
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "FREEZE (rw) register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "AUXCNT (r) register accessor: an alias for `Reg<AUXCNT_SPEC>`"]
pub type AUXCNT = crate::Reg<auxcnt::AUXCNT_SPEC>;
#[doc = "Auxiliary Counter Value Register"]
pub mod auxcnt;
#[doc = "INPUT (rw) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "PCNT Input Register"]
pub mod input;
#[doc = "OVSCFG (rw) register accessor: an alias for `Reg<OVSCFG_SPEC>`"]
pub type OVSCFG = crate::Reg<ovscfg::OVSCFG_SPEC>;
#[doc = "Oversampling Config Register"]
pub mod ovscfg;
