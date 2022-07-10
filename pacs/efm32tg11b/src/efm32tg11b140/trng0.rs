#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x04 - FIFO Level Register"]
    pub fifolevel: crate::Reg<fifolevel::FIFOLEVEL_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FIFO Depth Register"]
    pub fifodepth: crate::Reg<fifodepth::FIFODEPTH_SPEC>,
    #[doc = "0x10 - Key Register 0"]
    pub key0: crate::Reg<key0::KEY0_SPEC>,
    #[doc = "0x14 - Key Register 1"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x18 - Key Register 2"]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    #[doc = "0x1c - Key Register 3"]
    pub key3: crate::Reg<key3::KEY3_SPEC>,
    #[doc = "0x20 - Test Data Register"]
    pub testdata: crate::Reg<testdata::TESTDATA_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x34 - Initial Wait Counter"]
    pub initwaitval: crate::Reg<initwaitval::INITWAITVAL_SPEC>,
    _reserved10: [u8; 0xc8],
    #[doc = "0x100 - FIFO Data"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
}
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Main Control Register"]
pub mod control;
#[doc = "FIFOLEVEL register accessor: an alias for `Reg<FIFOLEVEL_SPEC>`"]
pub type FIFOLEVEL = crate::Reg<fifolevel::FIFOLEVEL_SPEC>;
#[doc = "FIFO Level Register"]
pub mod fifolevel;
#[doc = "FIFODEPTH register accessor: an alias for `Reg<FIFODEPTH_SPEC>`"]
pub type FIFODEPTH = crate::Reg<fifodepth::FIFODEPTH_SPEC>;
#[doc = "FIFO Depth Register"]
pub mod fifodepth;
#[doc = "KEY0 register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Key Register 0"]
pub mod key0;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Key Register 1"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "Key Register 2"]
pub mod key2;
#[doc = "KEY3 register accessor: an alias for `Reg<KEY3_SPEC>`"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "Key Register 3"]
pub mod key3;
#[doc = "TESTDATA register accessor: an alias for `Reg<TESTDATA_SPEC>`"]
pub type TESTDATA = crate::Reg<testdata::TESTDATA_SPEC>;
#[doc = "Test Data Register"]
pub mod testdata;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INITWAITVAL register accessor: an alias for `Reg<INITWAITVAL_SPEC>`"]
pub type INITWAITVAL = crate::Reg<initwaitval::INITWAITVAL_SPEC>;
#[doc = "Initial Wait Counter"]
pub mod initwaitval;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Data"]
pub mod fifo;
