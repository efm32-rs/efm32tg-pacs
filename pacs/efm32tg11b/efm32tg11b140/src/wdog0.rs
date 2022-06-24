#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x08 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x0c - PRS Control Register"]
    pub pch0_prsctrl: crate::Reg<pch0_prsctrl::PCH0_PRSCTRL_SPEC>,
    #[doc = "0x10 - PRS Control Register"]
    pub pch1_prsctrl: crate::Reg<pch1_prsctrl::PCH1_PRSCTRL_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - Watchdog Interrupt Flags"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "PCH0_PRSCTRL register accessor: an alias for `Reg<PCH0_PRSCTRL_SPEC>`"]
pub type PCH0_PRSCTRL = crate::Reg<pch0_prsctrl::PCH0_PRSCTRL_SPEC>;
#[doc = "PRS Control Register"]
pub mod pch0_prsctrl;
#[doc = "PCH1_PRSCTRL register accessor: an alias for `Reg<PCH1_PRSCTRL_SPEC>`"]
pub type PCH1_PRSCTRL = crate::Reg<pch1_prsctrl::PCH1_PRSCTRL_SPEC>;
#[doc = "PRS Control Register"]
pub mod pch1_prsctrl;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Watchdog Interrupt Flags"]
pub mod if_;
#[doc = "IFS register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
