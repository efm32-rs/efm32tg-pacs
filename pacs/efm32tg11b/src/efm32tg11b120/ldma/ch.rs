#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Channel Peripheral Request Select Register"]
    pub reqsel: REQSEL,
    #[doc = "0x04 - Channel Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Channel Loop Counter Register"]
    pub loop_: LOOP,
    #[doc = "0x0c - Channel Descriptor Control Word Register"]
    pub ctrl: CTRL,
    #[doc = "0x10 - Channel Descriptor Source Data Address Register"]
    pub src: SRC,
    #[doc = "0x14 - Channel Descriptor Destination Data Address Register"]
    pub dst: DST,
    #[doc = "0x18 - Channel Descriptor Link Structure Address Register"]
    pub link: LINK,
}
#[doc = "REQSEL (rw) register accessor: an alias for `Reg<REQSEL_SPEC>`"]
pub type REQSEL = crate::Reg<reqsel::REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod reqsel;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod cfg;
#[doc = "LOOP (rw) register accessor: an alias for `Reg<LOOP_SPEC>`"]
pub type LOOP = crate::Reg<loop_::LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod loop_;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ctrl;
#[doc = "SRC (rw) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod src;
#[doc = "DST (rw) register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod dst;
#[doc = "LINK (rw) register accessor: an alias for `Reg<LINK_SPEC>`"]
pub type LINK = crate::Reg<link::LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod link;
