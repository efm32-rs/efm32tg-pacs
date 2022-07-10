#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x10 - Single Channel Control Register"]
    pub singlectrl: crate::Reg<singlectrl::SINGLECTRL_SPEC>,
    #[doc = "0x14 - Single Channel Control Register Continued"]
    pub singlectrlx: crate::Reg<singlectrlx::SINGLECTRLX_SPEC>,
    #[doc = "0x18 - Scan Control Register"]
    pub scanctrl: crate::Reg<scanctrl::SCANCTRL_SPEC>,
    #[doc = "0x1c - Scan Control Register Continued"]
    pub scanctrlx: crate::Reg<scanctrlx::SCANCTRLX_SPEC>,
    #[doc = "0x20 - Scan Sequence Input Mask Register"]
    pub scanmask: crate::Reg<scanmask::SCANMASK_SPEC>,
    #[doc = "0x24 - Input Selection Register for Scan Mode"]
    pub scaninputsel: crate::Reg<scaninputsel::SCANINPUTSEL_SPEC>,
    #[doc = "0x28 - Negative Input Select Register for Scan"]
    pub scannegsel: crate::Reg<scannegsel::SCANNEGSEL_SPEC>,
    #[doc = "0x2c - Compare Threshold Register"]
    pub cmpthr: crate::Reg<cmpthr::CMPTHR_SPEC>,
    #[doc = "0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
    pub biasprog: crate::Reg<biasprog::BIASPROG_SPEC>,
    #[doc = "0x34 - Calibration Register"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x3c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x40 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x48 - Single Conversion Result Data"]
    pub singledata: crate::Reg<singledata::SINGLEDATA_SPEC>,
    #[doc = "0x4c - Scan Conversion Result Data"]
    pub scandata: crate::Reg<scandata::SCANDATA_SPEC>,
    #[doc = "0x50 - Single Conversion Result Data Peek Register"]
    pub singledatap: crate::Reg<singledatap::SINGLEDATAP_SPEC>,
    #[doc = "0x54 - Scan Sequence Result Data Peek Register"]
    pub scandatap: crate::Reg<scandatap::SCANDATAP_SPEC>,
    _reserved21: [u8; 0x10],
    #[doc = "0x68 - Scan Sequence Result Data + Data Source Register"]
    pub scandatax: crate::Reg<scandatax::SCANDATAX_SPEC>,
    #[doc = "0x6c - Scan Sequence Result Data + Data Source Peek Register"]
    pub scandataxp: crate::Reg<scandataxp::SCANDATAXP_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x7c - APORT Request Status Register"]
    pub aportreq: crate::Reg<aportreq::APORTREQ_SPEC>,
    #[doc = "0x80 - APORT Conflict Status Register"]
    pub aportconflict: crate::Reg<aportconflict::APORTCONFLICT_SPEC>,
    #[doc = "0x84 - Single FIFO Count Register"]
    pub singlefifocount: crate::Reg<singlefifocount::SINGLEFIFOCOUNT_SPEC>,
    #[doc = "0x88 - Scan FIFO Count Register"]
    pub scanfifocount: crate::Reg<scanfifocount::SCANFIFOCOUNT_SPEC>,
    #[doc = "0x8c - Single FIFO Clear Register"]
    pub singlefifoclear: crate::Reg<singlefifoclear::SINGLEFIFOCLEAR_SPEC>,
    #[doc = "0x90 - Scan FIFO Clear Register"]
    pub scanfifoclear: crate::Reg<scanfifoclear::SCANFIFOCLEAR_SPEC>,
    #[doc = "0x94 - APORT Bus Master Disable Register"]
    pub aportmasterdis: crate::Reg<aportmasterdis::APORTMASTERDIS_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SINGLECTRL register accessor: an alias for `Reg<SINGLECTRL_SPEC>`"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Channel Control Register"]
pub mod singlectrl;
#[doc = "SINGLECTRLX register accessor: an alias for `Reg<SINGLECTRLX_SPEC>`"]
pub type SINGLECTRLX = crate::Reg<singlectrlx::SINGLECTRLX_SPEC>;
#[doc = "Single Channel Control Register Continued"]
pub mod singlectrlx;
#[doc = "SCANCTRL register accessor: an alias for `Reg<SCANCTRL_SPEC>`"]
pub type SCANCTRL = crate::Reg<scanctrl::SCANCTRL_SPEC>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "SCANCTRLX register accessor: an alias for `Reg<SCANCTRLX_SPEC>`"]
pub type SCANCTRLX = crate::Reg<scanctrlx::SCANCTRLX_SPEC>;
#[doc = "Scan Control Register Continued"]
pub mod scanctrlx;
#[doc = "SCANMASK register accessor: an alias for `Reg<SCANMASK_SPEC>`"]
pub type SCANMASK = crate::Reg<scanmask::SCANMASK_SPEC>;
#[doc = "Scan Sequence Input Mask Register"]
pub mod scanmask;
#[doc = "SCANINPUTSEL register accessor: an alias for `Reg<SCANINPUTSEL_SPEC>`"]
pub type SCANINPUTSEL = crate::Reg<scaninputsel::SCANINPUTSEL_SPEC>;
#[doc = "Input Selection Register for Scan Mode"]
pub mod scaninputsel;
#[doc = "SCANNEGSEL register accessor: an alias for `Reg<SCANNEGSEL_SPEC>`"]
pub type SCANNEGSEL = crate::Reg<scannegsel::SCANNEGSEL_SPEC>;
#[doc = "Negative Input Select Register for Scan"]
pub mod scannegsel;
#[doc = "CMPTHR register accessor: an alias for `Reg<CMPTHR_SPEC>`"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "Compare Threshold Register"]
pub mod cmpthr;
#[doc = "BIASPROG register accessor: an alias for `Reg<BIASPROG_SPEC>`"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub mod biasprog;
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "SINGLEDATA register accessor: an alias for `Reg<SINGLEDATA_SPEC>`"]
pub type SINGLEDATA = crate::Reg<singledata::SINGLEDATA_SPEC>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA register accessor: an alias for `Reg<SCANDATA_SPEC>`"]
pub type SCANDATA = crate::Reg<scandata::SCANDATA_SPEC>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP register accessor: an alias for `Reg<SINGLEDATAP_SPEC>`"]
pub type SINGLEDATAP = crate::Reg<singledatap::SINGLEDATAP_SPEC>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP register accessor: an alias for `Reg<SCANDATAP_SPEC>`"]
pub type SCANDATAP = crate::Reg<scandatap::SCANDATAP_SPEC>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "SCANDATAX register accessor: an alias for `Reg<SCANDATAX_SPEC>`"]
pub type SCANDATAX = crate::Reg<scandatax::SCANDATAX_SPEC>;
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub mod scandatax;
#[doc = "SCANDATAXP register accessor: an alias for `Reg<SCANDATAXP_SPEC>`"]
pub type SCANDATAXP = crate::Reg<scandataxp::SCANDATAXP_SPEC>;
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub mod scandataxp;
#[doc = "APORTREQ register accessor: an alias for `Reg<APORTREQ_SPEC>`"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT register accessor: an alias for `Reg<APORTCONFLICT_SPEC>`"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "SINGLEFIFOCOUNT register accessor: an alias for `Reg<SINGLEFIFOCOUNT_SPEC>`"]
pub type SINGLEFIFOCOUNT = crate::Reg<singlefifocount::SINGLEFIFOCOUNT_SPEC>;
#[doc = "Single FIFO Count Register"]
pub mod singlefifocount;
#[doc = "SCANFIFOCOUNT register accessor: an alias for `Reg<SCANFIFOCOUNT_SPEC>`"]
pub type SCANFIFOCOUNT = crate::Reg<scanfifocount::SCANFIFOCOUNT_SPEC>;
#[doc = "Scan FIFO Count Register"]
pub mod scanfifocount;
#[doc = "SINGLEFIFOCLEAR register accessor: an alias for `Reg<SINGLEFIFOCLEAR_SPEC>`"]
pub type SINGLEFIFOCLEAR = crate::Reg<singlefifoclear::SINGLEFIFOCLEAR_SPEC>;
#[doc = "Single FIFO Clear Register"]
pub mod singlefifoclear;
#[doc = "SCANFIFOCLEAR register accessor: an alias for `Reg<SCANFIFOCLEAR_SPEC>`"]
pub type SCANFIFOCLEAR = crate::Reg<scanfifoclear::SCANFIFOCLEAR_SPEC>;
#[doc = "Scan FIFO Clear Register"]
pub mod scanfifoclear;
#[doc = "APORTMASTERDIS register accessor: an alias for `Reg<APORTMASTERDIS_SPEC>`"]
pub type APORTMASTERDIS = crate::Reg<aportmasterdis::APORTMASTERDIS_SPEC>;
#[doc = "APORT Bus Master Disable Register"]
pub mod aportmasterdis;
