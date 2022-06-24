#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MTB Trace Position Register."]
    pub position: crate::Reg<position::POSITION_SPEC>,
    #[doc = "0x04 - MTB Trace Control Register"]
    pub master: crate::Reg<master::MASTER_SPEC>,
    #[doc = "0x08 - MTB Trace Flow Register"]
    pub flow: crate::Reg<flow::FLOW_SPEC>,
    #[doc = "0x0c - MTB Trace Base Register"]
    pub base: crate::Reg<base::BASE_SPEC>,
}
#[doc = "POSITION register accessor: an alias for `Reg<POSITION_SPEC>`"]
pub type POSITION = crate::Reg<position::POSITION_SPEC>;
#[doc = "MTB Trace Position Register."]
pub mod position;
#[doc = "MASTER register accessor: an alias for `Reg<MASTER_SPEC>`"]
pub type MASTER = crate::Reg<master::MASTER_SPEC>;
#[doc = "MTB Trace Control Register"]
pub mod master;
#[doc = "FLOW register accessor: an alias for `Reg<FLOW_SPEC>`"]
pub type FLOW = crate::Reg<flow::FLOW_SPEC>;
#[doc = "MTB Trace Flow Register"]
pub mod flow;
#[doc = "BASE register accessor: an alias for `Reg<BASE_SPEC>`"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "MTB Trace Base Register"]
pub mod base;
