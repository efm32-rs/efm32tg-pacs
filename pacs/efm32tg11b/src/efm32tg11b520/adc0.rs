#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Single Channel Control Register"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x14 - Single Channel Control Register Continued"]
    pub singlectrlx: SINGLECTRLX,
    #[doc = "0x18 - Scan Control Register"]
    pub scanctrl: SCANCTRL,
    #[doc = "0x1c - Scan Control Register Continued"]
    pub scanctrlx: SCANCTRLX,
    #[doc = "0x20 - Scan Sequence Input Mask Register"]
    pub scanmask: SCANMASK,
    #[doc = "0x24 - Input Selection Register for Scan Mode"]
    pub scaninputsel: SCANINPUTSEL,
    #[doc = "0x28 - Negative Input Select Register for Scan"]
    pub scannegsel: SCANNEGSEL,
    #[doc = "0x2c - Compare Threshold Register"]
    pub cmpthr: CMPTHR,
    #[doc = "0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
    pub biasprog: BIASPROG,
    #[doc = "0x34 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x3c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x40 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x48 - Single Conversion Result Data"]
    pub singledata: SINGLEDATA,
    #[doc = "0x4c - Scan Conversion Result Data"]
    pub scandata: SCANDATA,
    #[doc = "0x50 - Single Conversion Result Data Peek Register"]
    pub singledatap: SINGLEDATAP,
    #[doc = "0x54 - Scan Sequence Result Data Peek Register"]
    pub scandatap: SCANDATAP,
    _reserved21: [u8; 0x10],
    #[doc = "0x68 - Scan Sequence Result Data + Data Source Register"]
    pub scandatax: SCANDATAX,
    #[doc = "0x6c - Scan Sequence Result Data + Data Source Peek Register"]
    pub scandataxp: SCANDATAXP,
    _reserved23: [u8; 0x0c],
    #[doc = "0x7c - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x80 - APORT Conflict Status Register"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x84 - Single FIFO Count Register"]
    pub singlefifocount: SINGLEFIFOCOUNT,
    #[doc = "0x88 - Scan FIFO Count Register"]
    pub scanfifocount: SCANFIFOCOUNT,
    #[doc = "0x8c - Single FIFO Clear Register"]
    pub singlefifoclear: SINGLEFIFOCLEAR,
    #[doc = "0x90 - Scan FIFO Clear Register"]
    pub scanfifoclear: SCANFIFOCLEAR,
    #[doc = "0x94 - APORT Bus Master Disable Register"]
    pub aportmasterdis: APORTMASTERDIS,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SINGLECTRL (rw) register accessor: an alias for `Reg<SINGLECTRL_SPEC>`"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Channel Control Register"]
pub mod singlectrl;
#[doc = "SINGLECTRLX (rw) register accessor: an alias for `Reg<SINGLECTRLX_SPEC>`"]
pub type SINGLECTRLX = crate::Reg<singlectrlx::SINGLECTRLX_SPEC>;
#[doc = "Single Channel Control Register Continued"]
pub mod singlectrlx;
#[doc = "SCANCTRL (rw) register accessor: an alias for `Reg<SCANCTRL_SPEC>`"]
pub type SCANCTRL = crate::Reg<scanctrl::SCANCTRL_SPEC>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "SCANCTRLX (rw) register accessor: an alias for `Reg<SCANCTRLX_SPEC>`"]
pub type SCANCTRLX = crate::Reg<scanctrlx::SCANCTRLX_SPEC>;
#[doc = "Scan Control Register Continued"]
pub mod scanctrlx;
#[doc = "SCANMASK (rw) register accessor: an alias for `Reg<SCANMASK_SPEC>`"]
pub type SCANMASK = crate::Reg<scanmask::SCANMASK_SPEC>;
#[doc = "Scan Sequence Input Mask Register"]
pub mod scanmask;
#[doc = "SCANINPUTSEL (rw) register accessor: an alias for `Reg<SCANINPUTSEL_SPEC>`"]
pub type SCANINPUTSEL = crate::Reg<scaninputsel::SCANINPUTSEL_SPEC>;
#[doc = "Input Selection Register for Scan Mode"]
pub mod scaninputsel;
#[doc = "SCANNEGSEL (rw) register accessor: an alias for `Reg<SCANNEGSEL_SPEC>`"]
pub type SCANNEGSEL = crate::Reg<scannegsel::SCANNEGSEL_SPEC>;
#[doc = "Negative Input Select Register for Scan"]
pub mod scannegsel;
#[doc = "CMPTHR (rw) register accessor: an alias for `Reg<CMPTHR_SPEC>`"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "Compare Threshold Register"]
pub mod cmpthr;
#[doc = "BIASPROG (rw) register accessor: an alias for `Reg<BIASPROG_SPEC>`"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub mod biasprog;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
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
#[doc = "SINGLEDATA (r) register accessor: an alias for `Reg<SINGLEDATA_SPEC>`"]
pub type SINGLEDATA = crate::Reg<singledata::SINGLEDATA_SPEC>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA (r) register accessor: an alias for `Reg<SCANDATA_SPEC>`"]
pub type SCANDATA = crate::Reg<scandata::SCANDATA_SPEC>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP (r) register accessor: an alias for `Reg<SINGLEDATAP_SPEC>`"]
pub type SINGLEDATAP = crate::Reg<singledatap::SINGLEDATAP_SPEC>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP (r) register accessor: an alias for `Reg<SCANDATAP_SPEC>`"]
pub type SCANDATAP = crate::Reg<scandatap::SCANDATAP_SPEC>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "SCANDATAX (r) register accessor: an alias for `Reg<SCANDATAX_SPEC>`"]
pub type SCANDATAX = crate::Reg<scandatax::SCANDATAX_SPEC>;
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub mod scandatax;
#[doc = "SCANDATAXP (r) register accessor: an alias for `Reg<SCANDATAXP_SPEC>`"]
pub type SCANDATAXP = crate::Reg<scandataxp::SCANDATAXP_SPEC>;
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub mod scandataxp;
#[doc = "APORTREQ (r) register accessor: an alias for `Reg<APORTREQ_SPEC>`"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT (r) register accessor: an alias for `Reg<APORTCONFLICT_SPEC>`"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "SINGLEFIFOCOUNT (r) register accessor: an alias for `Reg<SINGLEFIFOCOUNT_SPEC>`"]
pub type SINGLEFIFOCOUNT = crate::Reg<singlefifocount::SINGLEFIFOCOUNT_SPEC>;
#[doc = "Single FIFO Count Register"]
pub mod singlefifocount;
#[doc = "SCANFIFOCOUNT (r) register accessor: an alias for `Reg<SCANFIFOCOUNT_SPEC>`"]
pub type SCANFIFOCOUNT = crate::Reg<scanfifocount::SCANFIFOCOUNT_SPEC>;
#[doc = "Scan FIFO Count Register"]
pub mod scanfifocount;
#[doc = "SINGLEFIFOCLEAR (w) register accessor: an alias for `Reg<SINGLEFIFOCLEAR_SPEC>`"]
pub type SINGLEFIFOCLEAR = crate::Reg<singlefifoclear::SINGLEFIFOCLEAR_SPEC>;
#[doc = "Single FIFO Clear Register"]
pub mod singlefifoclear;
#[doc = "SCANFIFOCLEAR (w) register accessor: an alias for `Reg<SCANFIFOCLEAR_SPEC>`"]
pub type SCANFIFOCLEAR = crate::Reg<scanfifoclear::SCANFIFOCLEAR_SPEC>;
#[doc = "Scan FIFO Clear Register"]
pub mod scanfifoclear;
#[doc = "APORTMASTERDIS (rw) register accessor: an alias for `Reg<APORTMASTERDIS_SPEC>`"]
pub type APORTMASTERDIS = crate::Reg<aportmasterdis::APORTMASTERDIS_SPEC>;
#[doc = "APORT Bus Master Disable Register"]
pub mod aportmasterdis;
