#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - State Register"]
    pub state: STATE,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Clock Division Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x14 - Slave Address Register"]
    pub saddr: SADDR,
    #[doc = "0x18 - Slave Address Mask Register"]
    pub saddrmask: SADDRMASK,
    #[doc = "0x1c - Receive Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - Receive Buffer Double Data Register"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x24 - Receive Buffer Data Peek Register"]
    pub rxdatap: RXDATAP,
    #[doc = "0x28 - Receive Buffer Double Data Peek Register"]
    pub rxdoublep: RXDOUBLEP,
    #[doc = "0x2c - Transmit Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x30 - Transmit Buffer Double Data Register"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x34 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x38 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x3c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x44 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x48 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATE (r) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "State Register"]
pub mod state;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Division Register"]
pub mod clkdiv;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "Slave Address Register"]
pub mod saddr;
#[doc = "SADDRMASK (rw) register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "Slave Address Mask Register"]
pub mod saddrmask;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDOUBLE (r) register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "Receive Buffer Double Data Register"]
pub mod rxdouble;
#[doc = "RXDATAP (r) register accessor: an alias for `Reg<RXDATAP_SPEC>`"]
pub type RXDATAP = crate::Reg<rxdatap::RXDATAP_SPEC>;
#[doc = "Receive Buffer Data Peek Register"]
pub mod rxdatap;
#[doc = "RXDOUBLEP (r) register accessor: an alias for `Reg<RXDOUBLEP_SPEC>`"]
pub type RXDOUBLEP = crate::Reg<rxdoublep::RXDOUBLEP_SPEC>;
#[doc = "Receive Buffer Double Data Peek Register"]
pub mod rxdoublep;
#[doc = "TXDATA (rw) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Buffer Data Register"]
pub mod txdata;
#[doc = "TXDOUBLE (rw) register accessor: an alias for `Reg<TXDOUBLE_SPEC>`"]
pub type TXDOUBLE = crate::Reg<txdouble::TXDOUBLE_SPEC>;
#[doc = "Transmit Buffer Double Data Register"]
pub mod txdouble;
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
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
