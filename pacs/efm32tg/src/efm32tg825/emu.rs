#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x24 - Auxiliary Control Register"]
    pub auxctrl: crate::Reg<auxctrl::AUXCTRL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "AUXCTRL register accessor: an alias for `Reg<AUXCTRL_SPEC>`"]
pub type AUXCTRL = crate::Reg<auxctrl::AUXCTRL_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod auxctrl;
