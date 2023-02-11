#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - USART Frame Format Register"]
    pub frame: FRAME,
    #[doc = "0x08 - USART Trigger Control Register"]
    pub trigctrl: TRIGCTRL,
    #[doc = "0x0c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x10 - USART Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Clock Control Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    pub rxdatax: RXDATAX,
    #[doc = "0x1c - RX Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    pub rxdoublex: RXDOUBLEX,
    #[doc = "0x24 - RX FIFO Double Data Register"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    pub rxdataxp: RXDATAXP,
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    pub rxdoublexp: RXDOUBLEXP,
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    pub txdatax: TXDATAX,
    #[doc = "0x34 - TX Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    pub txdoublex: TXDOUBLEX,
    #[doc = "0x3c - TX Buffer Double Data Register"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x40 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - IrDA Control Register"]
    pub irctrl: IRCTRL,
    _reserved21: [u8; 0x04],
    #[doc = "0x58 - USART Input Register"]
    pub input: INPUT,
    #[doc = "0x5c - I2S Control Register"]
    pub i2sctrl: I2SCTRL,
    #[doc = "0x60 - Timing Register"]
    pub timing: TIMING,
    #[doc = "0x64 - Control Register Extended"]
    pub ctrlx: CTRLX,
    #[doc = "0x68 - Used to Generate Interrupts and Various Delays"]
    pub timecmp0: TIMECMP0,
    #[doc = "0x6c - Used to Generate Interrupts and Various Delays"]
    pub timecmp1: TIMECMP1,
    #[doc = "0x70 - Used to Generate Interrupts and Various Delays"]
    pub timecmp2: TIMECMP2,
    #[doc = "0x74 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x78 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x7c - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FRAME (rw) register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "TRIGCTRL (rw) register accessor: an alias for `Reg<TRIGCTRL_SPEC>`"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "USART Trigger Control Register"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RXDATAX (r) register accessor: an alias for `Reg<RXDATAX_SPEC>`"]
pub type RXDATAX = crate::Reg<rxdatax::RXDATAX_SPEC>;
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RXDOUBLEX (r) register accessor: an alias for `Reg<RXDOUBLEX_SPEC>`"]
pub type RXDOUBLEX = crate::Reg<rxdoublex::RXDOUBLEX_SPEC>;
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RXDOUBLE (r) register accessor: an alias for `Reg<RXDOUBLE_SPEC>`"]
pub type RXDOUBLE = crate::Reg<rxdouble::RXDOUBLE_SPEC>;
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RXDATAXP (r) register accessor: an alias for `Reg<RXDATAXP_SPEC>`"]
pub type RXDATAXP = crate::Reg<rxdataxp::RXDATAXP_SPEC>;
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP (r) register accessor: an alias for `Reg<RXDOUBLEXP_SPEC>`"]
pub type RXDOUBLEXP = crate::Reg<rxdoublexp::RXDOUBLEXP_SPEC>;
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TXDATAX (rw) register accessor: an alias for `Reg<TXDATAX_SPEC>`"]
pub type TXDATAX = crate::Reg<txdatax::TXDATAX_SPEC>;
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TXDATA (rw) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TXDOUBLEX (rw) register accessor: an alias for `Reg<TXDOUBLEX_SPEC>`"]
pub type TXDOUBLEX = crate::Reg<txdoublex::TXDOUBLEX_SPEC>;
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TXDOUBLE (rw) register accessor: an alias for `Reg<TXDOUBLE_SPEC>`"]
pub type TXDOUBLE = crate::Reg<txdouble::TXDOUBLE_SPEC>;
#[doc = "TX Buffer Double Data Register"]
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
#[doc = "IRCTRL (rw) register accessor: an alias for `Reg<IRCTRL_SPEC>`"]
pub type IRCTRL = crate::Reg<irctrl::IRCTRL_SPEC>;
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "INPUT (rw) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2SCTRL (rw) register accessor: an alias for `Reg<I2SCTRL_SPEC>`"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "I2S Control Register"]
pub mod i2sctrl;
#[doc = "TIMING (rw) register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "Timing Register"]
pub mod timing;
#[doc = "CTRLX (rw) register accessor: an alias for `Reg<CTRLX_SPEC>`"]
pub type CTRLX = crate::Reg<ctrlx::CTRLX_SPEC>;
#[doc = "Control Register Extended"]
pub mod ctrlx;
#[doc = "TIMECMP0 (rw) register accessor: an alias for `Reg<TIMECMP0_SPEC>`"]
pub type TIMECMP0 = crate::Reg<timecmp0::TIMECMP0_SPEC>;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp0;
#[doc = "TIMECMP1 (rw) register accessor: an alias for `Reg<TIMECMP1_SPEC>`"]
pub type TIMECMP1 = crate::Reg<timecmp1::TIMECMP1_SPEC>;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp1;
#[doc = "TIMECMP2 (rw) register accessor: an alias for `Reg<TIMECMP2_SPEC>`"]
pub type TIMECMP2 = crate::Reg<timecmp2::TIMECMP2_SPEC>;
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp2;
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
