#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Input Selection Register"]
    pub inputsel: crate::Reg<inputsel::INPUTSEL_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - APORT Request Status Register"]
    pub aportreq: crate::Reg<aportreq::APORTREQ_SPEC>,
    #[doc = "0x24 - APORT Conflict Status Register"]
    pub aportconflict: crate::Reg<aportconflict::APORTCONFLICT_SPEC>,
    #[doc = "0x28 - Hysteresis 0 Register"]
    pub hysteresis0: crate::Reg<hysteresis0::HYSTERESIS0_SPEC>,
    #[doc = "0x2c - Hysteresis 1 Register"]
    pub hysteresis1: crate::Reg<hysteresis1::HYSTERESIS1_SPEC>,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - I/O Routing Pine Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
    #[doc = "0x44 - I/O Routing Location Register"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
    #[doc = "0x48 - External Override Interface Control"]
    pub extifctrl: crate::Reg<extifctrl::EXTIFCTRL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "INPUTSEL register accessor: an alias for `Reg<INPUTSEL_SPEC>`"]
pub type INPUTSEL = crate::Reg<inputsel::INPUTSEL_SPEC>;
#[doc = "Input Selection Register"]
pub mod inputsel;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
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
#[doc = "APORTREQ register accessor: an alias for `Reg<APORTREQ_SPEC>`"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT register accessor: an alias for `Reg<APORTCONFLICT_SPEC>`"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "HYSTERESIS0 register accessor: an alias for `Reg<HYSTERESIS0_SPEC>`"]
pub type HYSTERESIS0 = crate::Reg<hysteresis0::HYSTERESIS0_SPEC>;
#[doc = "Hysteresis 0 Register"]
pub mod hysteresis0;
#[doc = "HYSTERESIS1 register accessor: an alias for `Reg<HYSTERESIS1_SPEC>`"]
pub type HYSTERESIS1 = crate::Reg<hysteresis1::HYSTERESIS1_SPEC>;
#[doc = "Hysteresis 1 Register"]
pub mod hysteresis1;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pine Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "EXTIFCTRL register accessor: an alias for `Reg<EXTIFCTRL_SPEC>`"]
pub type EXTIFCTRL = crate::Reg<extifctrl::EXTIFCTRL_SPEC>;
#[doc = "External Override Interface Control"]
pub mod extifctrl;
