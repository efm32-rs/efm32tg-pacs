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
    #[doc = "0x10 - Compare Value Register 0"]
    pub comp0: COMP0,
    #[doc = "0x14 - Compare Value Register 1"]
    pub comp1: COMP1,
    #[doc = "0x18 - Repeat Counter Register 0"]
    pub rep0: REP0,
    #[doc = "0x1c - Repeat Counter Register 1"]
    pub rep1: REP1,
    #[doc = "0x20 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x24 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x28 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x2c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved13: [u8; 0x08],
    #[doc = "0x40 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x44 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved15: [u8; 0x08],
    #[doc = "0x50 - PRS Input Select Register"]
    pub prssel: PRSSEL,
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
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "COMP0 (rw) register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Compare Value Register 0"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Compare Value Register 1"]
pub mod comp1;
#[doc = "REP0 (rw) register accessor: an alias for `Reg<REP0_SPEC>`"]
pub type REP0 = crate::Reg<rep0::REP0_SPEC>;
#[doc = "Repeat Counter Register 0"]
pub mod rep0;
#[doc = "REP1 (rw) register accessor: an alias for `Reg<REP1_SPEC>`"]
pub type REP1 = crate::Reg<rep1::REP1_SPEC>;
#[doc = "Repeat Counter Register 1"]
pub mod rep1;
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
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "PRSSEL (rw) register accessor: an alias for `Reg<PRSSEL_SPEC>`"]
pub type PRSSEL = crate::Reg<prssel::PRSSEL_SPEC>;
#[doc = "PRS Input Select Register"]
pub mod prssel;
