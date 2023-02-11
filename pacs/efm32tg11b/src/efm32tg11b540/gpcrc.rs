#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - CRC Init Value"]
    pub init: INIT,
    #[doc = "0x0c - CRC Polynomial Value"]
    pub poly: POLY,
    #[doc = "0x10 - Input 32-bit Data Register"]
    pub inputdata: INPUTDATA,
    #[doc = "0x14 - Input 16-bit Data Register"]
    pub inputdatahword: INPUTDATAHWORD,
    #[doc = "0x18 - Input 8-bit Data Register"]
    pub inputdatabyte: INPUTDATABYTE,
    #[doc = "0x1c - CRC Data Register"]
    pub data: DATA,
    #[doc = "0x20 - CRC Data Reverse Register"]
    pub datarev: DATAREV,
    #[doc = "0x24 - CRC Data Byte Reverse Register"]
    pub databyterev: DATABYTEREV,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "CRC Init Value"]
pub mod init;
#[doc = "POLY (rw) register accessor: an alias for `Reg<POLY_SPEC>`"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "CRC Polynomial Value"]
pub mod poly;
#[doc = "INPUTDATA (rw) register accessor: an alias for `Reg<INPUTDATA_SPEC>`"]
pub type INPUTDATA = crate::Reg<inputdata::INPUTDATA_SPEC>;
#[doc = "Input 32-bit Data Register"]
pub mod inputdata;
#[doc = "INPUTDATAHWORD (rw) register accessor: an alias for `Reg<INPUTDATAHWORD_SPEC>`"]
pub type INPUTDATAHWORD = crate::Reg<inputdatahword::INPUTDATAHWORD_SPEC>;
#[doc = "Input 16-bit Data Register"]
pub mod inputdatahword;
#[doc = "INPUTDATABYTE (rw) register accessor: an alias for `Reg<INPUTDATABYTE_SPEC>`"]
pub type INPUTDATABYTE = crate::Reg<inputdatabyte::INPUTDATABYTE_SPEC>;
#[doc = "Input 8-bit Data Register"]
pub mod inputdatabyte;
#[doc = "DATA (r) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "CRC Data Register"]
pub mod data;
#[doc = "DATAREV (r) register accessor: an alias for `Reg<DATAREV_SPEC>`"]
pub type DATAREV = crate::Reg<datarev::DATAREV_SPEC>;
#[doc = "CRC Data Reverse Register"]
pub mod datarev;
#[doc = "DATABYTEREV (r) register accessor: an alias for `Reg<DATABYTEREV_SPEC>`"]
pub type DATABYTEREV = crate::Reg<databyterev::DATABYTEREV_SPEC>;
#[doc = "CRC Data Byte Reverse Register"]
pub mod databyterev;
