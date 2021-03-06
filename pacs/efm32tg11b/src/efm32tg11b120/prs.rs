#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: crate::Reg<swpulse::SWPULSE_SPEC>,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: crate::Reg<swlevel::SWLEVEL_SPEC>,
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - I/O Routing Location Register"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
    #[doc = "0x14 - I/O Routing Location Register"]
    pub routeloc1: crate::Reg<routeloc1::ROUTELOC1_SPEC>,
    _reserved5: [u8; 0x18],
    #[doc = "0x30 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x34 - DMA Request 0 Register"]
    pub dmareq0: crate::Reg<dmareq0::DMAREQ0_SPEC>,
    #[doc = "0x38 - DMA Request 1 Register"]
    pub dmareq1: crate::Reg<dmareq1::DMAREQ1_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - PRS Channel Values"]
    pub peek: crate::Reg<peek::PEEK_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x50 - Channel Control Register"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x54 - Channel Control Register"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0x58 - Channel Control Register"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0x5c - Channel Control Register"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0x60 - Channel Control Register"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x64 - Channel Control Register"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    #[doc = "0x68 - Channel Control Register"]
    pub ch6_ctrl: crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>,
    #[doc = "0x6c - Channel Control Register"]
    pub ch7_ctrl: crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>,
    _reserved17: [u8; 0x90],
    #[doc = "0x100 - MTB Trace Control Register"]
    pub tracectrl: crate::Reg<tracectrl::TRACECTRL_SPEC>,
}
#[doc = "SWPULSE register accessor: an alias for `Reg<SWPULSE_SPEC>`"]
pub type SWPULSE = crate::Reg<swpulse::SWPULSE_SPEC>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL register accessor: an alias for `Reg<SWLEVEL_SPEC>`"]
pub type SWLEVEL = crate::Reg<swlevel::SWLEVEL_SPEC>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMAREQ0 register accessor: an alias for `Reg<DMAREQ0_SPEC>`"]
pub type DMAREQ0 = crate::Reg<dmareq0::DMAREQ0_SPEC>;
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMAREQ1 register accessor: an alias for `Reg<DMAREQ1_SPEC>`"]
pub type DMAREQ1 = crate::Reg<dmareq1::DMAREQ1_SPEC>;
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PEEK register accessor: an alias for `Reg<PEEK_SPEC>`"]
pub type PEEK = crate::Reg<peek::PEEK_SPEC>;
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "TRACECTRL register accessor: an alias for `Reg<TRACECTRL_SPEC>`"]
pub type TRACECTRL = crate::Reg<tracectrl::TRACECTRL_SPEC>;
#[doc = "MTB Trace Control Register"]
pub mod tracectrl;
