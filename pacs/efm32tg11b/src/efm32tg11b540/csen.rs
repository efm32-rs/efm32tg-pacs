#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Timing Control"]
    pub timctrl: crate::Reg<timctrl::TIMCTRL_SPEC>,
    #[doc = "0x08 - Command"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x0c - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - PRS Select"]
    pub prssel: crate::Reg<prssel::PRSSEL_SPEC>,
    #[doc = "0x14 - Output Data"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x18 - Scan Channel Mask 0"]
    pub scanmask0: crate::Reg<scanmask0::SCANMASK0_SPEC>,
    #[doc = "0x1c - Scan Input Selection 0"]
    pub scaninputsel0: crate::Reg<scaninputsel0::SCANINPUTSEL0_SPEC>,
    #[doc = "0x20 - Scan Channel Mask 1"]
    pub scanmask1: crate::Reg<scanmask1::SCANMASK1_SPEC>,
    #[doc = "0x24 - Scan Input Selection 1"]
    pub scaninputsel1: crate::Reg<scaninputsel1::SCANINPUTSEL1_SPEC>,
    #[doc = "0x28 - APORT Request Status"]
    pub aportreq: crate::Reg<aportreq::APORTREQ_SPEC>,
    #[doc = "0x2c - APORT Request Conflict"]
    pub aportconflict: crate::Reg<aportconflict::APORTCONFLICT_SPEC>,
    #[doc = "0x30 - Comparator Threshold"]
    pub cmpthr: crate::Reg<cmpthr::CMPTHR_SPEC>,
    #[doc = "0x34 - Exponential Moving Average"]
    pub ema: crate::Reg<ema::EMA_SPEC>,
    #[doc = "0x38 - Exponential Moving Average Control"]
    pub emactrl: crate::Reg<emactrl::EMACTRL_SPEC>,
    #[doc = "0x3c - Single Conversion Control"]
    pub singlectrl: crate::Reg<singlectrl::SINGLECTRL_SPEC>,
    #[doc = "0x40 - Delta Modulation Baseline"]
    pub dmbaseline: crate::Reg<dmbaseline::DMBASELINE_SPEC>,
    #[doc = "0x44 - Delta Modulation Configuration"]
    pub dmcfg: crate::Reg<dmcfg::DMCFG_SPEC>,
    #[doc = "0x48 - Analog Control"]
    pub anactrl: crate::Reg<anactrl::ANACTRL_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x54 - Interrupt Flag"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x58 - Interrupt Flag Set"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x5c - Interrupt Flag Clear"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x60 - Interrupt Enable"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "TIMCTRL register accessor: an alias for `Reg<TIMCTRL_SPEC>`"]
pub type TIMCTRL = crate::Reg<timctrl::TIMCTRL_SPEC>;
#[doc = "Timing Control"]
pub mod timctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "PRSSEL register accessor: an alias for `Reg<PRSSEL_SPEC>`"]
pub type PRSSEL = crate::Reg<prssel::PRSSEL_SPEC>;
#[doc = "PRS Select"]
pub mod prssel;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Output Data"]
pub mod data;
#[doc = "SCANMASK0 register accessor: an alias for `Reg<SCANMASK0_SPEC>`"]
pub type SCANMASK0 = crate::Reg<scanmask0::SCANMASK0_SPEC>;
#[doc = "Scan Channel Mask 0"]
pub mod scanmask0;
#[doc = "SCANINPUTSEL0 register accessor: an alias for `Reg<SCANINPUTSEL0_SPEC>`"]
pub type SCANINPUTSEL0 = crate::Reg<scaninputsel0::SCANINPUTSEL0_SPEC>;
#[doc = "Scan Input Selection 0"]
pub mod scaninputsel0;
#[doc = "SCANMASK1 register accessor: an alias for `Reg<SCANMASK1_SPEC>`"]
pub type SCANMASK1 = crate::Reg<scanmask1::SCANMASK1_SPEC>;
#[doc = "Scan Channel Mask 1"]
pub mod scanmask1;
#[doc = "SCANINPUTSEL1 register accessor: an alias for `Reg<SCANINPUTSEL1_SPEC>`"]
pub type SCANINPUTSEL1 = crate::Reg<scaninputsel1::SCANINPUTSEL1_SPEC>;
#[doc = "Scan Input Selection 1"]
pub mod scaninputsel1;
#[doc = "APORTREQ register accessor: an alias for `Reg<APORTREQ_SPEC>`"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status"]
pub mod aportreq;
#[doc = "APORTCONFLICT register accessor: an alias for `Reg<APORTCONFLICT_SPEC>`"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Request Conflict"]
pub mod aportconflict;
#[doc = "CMPTHR register accessor: an alias for `Reg<CMPTHR_SPEC>`"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "EMA register accessor: an alias for `Reg<EMA_SPEC>`"]
pub type EMA = crate::Reg<ema::EMA_SPEC>;
#[doc = "Exponential Moving Average"]
pub mod ema;
#[doc = "EMACTRL register accessor: an alias for `Reg<EMACTRL_SPEC>`"]
pub type EMACTRL = crate::Reg<emactrl::EMACTRL_SPEC>;
#[doc = "Exponential Moving Average Control"]
pub mod emactrl;
#[doc = "SINGLECTRL register accessor: an alias for `Reg<SINGLECTRL_SPEC>`"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Conversion Control"]
pub mod singlectrl;
#[doc = "DMBASELINE register accessor: an alias for `Reg<DMBASELINE_SPEC>`"]
pub type DMBASELINE = crate::Reg<dmbaseline::DMBASELINE_SPEC>;
#[doc = "Delta Modulation Baseline"]
pub mod dmbaseline;
#[doc = "DMCFG register accessor: an alias for `Reg<DMCFG_SPEC>`"]
pub type DMCFG = crate::Reg<dmcfg::DMCFG_SPEC>;
#[doc = "Delta Modulation Configuration"]
pub mod dmcfg;
#[doc = "ANACTRL register accessor: an alias for `Reg<ANACTRL_SPEC>`"]
pub type ANACTRL = crate::Reg<anactrl::ANACTRL_SPEC>;
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IFS register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set"]
pub mod ifs;
#[doc = "IFC register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear"]
pub mod ifc;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ien;
