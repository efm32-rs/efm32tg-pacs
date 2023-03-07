#[doc = r"Register block"]
#[repr(C)]
pub struct CC {
    #[doc = "0x00 - CC Channel Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - CC Channel Value Register"]
    pub ccv: CCV,
    #[doc = "0x08 - CC Channel Value Peek Register"]
    pub ccvp: CCVP,
    #[doc = "0x0c - CC Channel Buffer Register"]
    pub ccvb: CCVB,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod ctrl;
#[doc = "CCV (rw) register accessor: an alias for `Reg<CCV_SPEC>`"]
pub type CCV = crate::Reg<ccv::CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod ccv;
#[doc = "CCVP (r) register accessor: an alias for `Reg<CCVP_SPEC>`"]
pub type CCVP = crate::Reg<ccvp::CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod ccvp;
#[doc = "CCVB (rw) register accessor: an alias for `Reg<CCVB_SPEC>`"]
pub type CCVB = crate::Reg<ccvb::CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod ccvb;
