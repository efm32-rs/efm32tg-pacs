#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Channel 0 Control Register"]
    pub ch0ctrl: CH0CTRL,
    #[doc = "0x0c - Channel 1 Control Register"]
    pub ch1ctrl: CH1CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x14 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x18 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x24 - Channel 0 Data Register"]
    pub ch0data: CH0DATA,
    #[doc = "0x28 - Channel 1 Data Register"]
    pub ch1data: CH1DATA,
    #[doc = "0x2c - Combined Data Register"]
    pub combdata: COMBDATA,
    #[doc = "0x30 - Calibration Register"]
    pub cal: CAL,
    _reserved13: [u8; 0x6c],
    #[doc = "0xa0 - Operational Amplifier APORT Request Status Register"]
    pub opa0_aportreq: OPA0_APORTREQ,
    #[doc = "0xa4 - Operational Amplifier APORT Conflict Status Register"]
    pub opa0_aportconflict: OPA0_APORTCONFLICT,
    #[doc = "0xa8 - Operational Amplifier Control Register"]
    pub opa0_ctrl: OPA0_CTRL,
    #[doc = "0xac - Operational Amplifier Timer Control Register"]
    pub opa0_timer: OPA0_TIMER,
    #[doc = "0xb0 - Operational Amplifier Mux Configuration Register"]
    pub opa0_mux: OPA0_MUX,
    #[doc = "0xb4 - Operational Amplifier Output Configuration Register"]
    pub opa0_out: OPA0_OUT,
    #[doc = "0xb8 - Operational Amplifier Calibration Register"]
    pub opa0_cal: OPA0_CAL,
    _reserved20: [u8; 0x04],
    #[doc = "0xc0 - Operational Amplifier APORT Request Status Register"]
    pub opa1_aportreq: OPA1_APORTREQ,
    #[doc = "0xc4 - Operational Amplifier APORT Conflict Status Register"]
    pub opa1_aportconflict: OPA1_APORTCONFLICT,
    #[doc = "0xc8 - Operational Amplifier Control Register"]
    pub opa1_ctrl: OPA1_CTRL,
    #[doc = "0xcc - Operational Amplifier Timer Control Register"]
    pub opa1_timer: OPA1_TIMER,
    #[doc = "0xd0 - Operational Amplifier Mux Configuration Register"]
    pub opa1_mux: OPA1_MUX,
    #[doc = "0xd4 - Operational Amplifier Output Configuration Register"]
    pub opa1_out: OPA1_OUT,
    #[doc = "0xd8 - Operational Amplifier Calibration Register"]
    pub opa1_cal: OPA1_CAL,
    _reserved27: [u8; 0x04],
    #[doc = "0xe0 - Operational Amplifier APORT Request Status Register"]
    pub opa2_aportreq: OPA2_APORTREQ,
    #[doc = "0xe4 - Operational Amplifier APORT Conflict Status Register"]
    pub opa2_aportconflict: OPA2_APORTCONFLICT,
    #[doc = "0xe8 - Operational Amplifier Control Register"]
    pub opa2_ctrl: OPA2_CTRL,
    #[doc = "0xec - Operational Amplifier Timer Control Register"]
    pub opa2_timer: OPA2_TIMER,
    #[doc = "0xf0 - Operational Amplifier Mux Configuration Register"]
    pub opa2_mux: OPA2_MUX,
    #[doc = "0xf4 - Operational Amplifier Output Configuration Register"]
    pub opa2_out: OPA2_OUT,
    #[doc = "0xf8 - Operational Amplifier Calibration Register"]
    pub opa2_cal: OPA2_CAL,
    _reserved34: [u8; 0x04],
    #[doc = "0x100 - Operational Amplifier APORT Request Status Register"]
    pub opa3_aportreq: OPA3_APORTREQ,
    #[doc = "0x104 - Operational Amplifier APORT Conflict Status Register"]
    pub opa3_aportconflict: OPA3_APORTCONFLICT,
    #[doc = "0x108 - Operational Amplifier Control Register"]
    pub opa3_ctrl: OPA3_CTRL,
    #[doc = "0x10c - Operational Amplifier Timer Control Register"]
    pub opa3_timer: OPA3_TIMER,
    #[doc = "0x110 - Operational Amplifier Mux Configuration Register"]
    pub opa3_mux: OPA3_MUX,
    #[doc = "0x114 - Operational Amplifier Output Configuration Register"]
    pub opa3_out: OPA3_OUT,
    #[doc = "0x118 - Operational Amplifier Calibration Register"]
    pub opa3_cal: OPA3_CAL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CH0CTRL (rw) register accessor: an alias for `Reg<CH0CTRL_SPEC>`"]
pub type CH0CTRL = crate::Reg<ch0ctrl::CH0CTRL_SPEC>;
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "CH1CTRL (rw) register accessor: an alias for `Reg<CH1CTRL_SPEC>`"]
pub type CH1CTRL = crate::Reg<ch1ctrl::CH1CTRL_SPEC>;
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
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
#[doc = "CH0DATA (rw) register accessor: an alias for `Reg<CH0DATA_SPEC>`"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "CH1DATA (rw) register accessor: an alias for `Reg<CH1DATA_SPEC>`"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "COMBDATA (rw) register accessor: an alias for `Reg<COMBDATA_SPEC>`"]
pub type COMBDATA = crate::Reg<combdata::COMBDATA_SPEC>;
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "OPA0_APORTREQ (r) register accessor: an alias for `Reg<OPA0_APORTREQ_SPEC>`"]
pub type OPA0_APORTREQ = crate::Reg<opa0_aportreq::OPA0_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa0_aportreq;
#[doc = "OPA0_APORTCONFLICT (r) register accessor: an alias for `Reg<OPA0_APORTCONFLICT_SPEC>`"]
pub type OPA0_APORTCONFLICT = crate::Reg<opa0_aportconflict::OPA0_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa0_aportconflict;
#[doc = "OPA0_CTRL (rw) register accessor: an alias for `Reg<OPA0_CTRL_SPEC>`"]
pub type OPA0_CTRL = crate::Reg<opa0_ctrl::OPA0_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa0_ctrl;
#[doc = "OPA0_TIMER (rw) register accessor: an alias for `Reg<OPA0_TIMER_SPEC>`"]
pub type OPA0_TIMER = crate::Reg<opa0_timer::OPA0_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa0_timer;
#[doc = "OPA0_MUX (rw) register accessor: an alias for `Reg<OPA0_MUX_SPEC>`"]
pub type OPA0_MUX = crate::Reg<opa0_mux::OPA0_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0_mux;
#[doc = "OPA0_OUT (rw) register accessor: an alias for `Reg<OPA0_OUT_SPEC>`"]
pub type OPA0_OUT = crate::Reg<opa0_out::OPA0_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa0_out;
#[doc = "OPA0_CAL (rw) register accessor: an alias for `Reg<OPA0_CAL_SPEC>`"]
pub type OPA0_CAL = crate::Reg<opa0_cal::OPA0_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa0_cal;
#[doc = "OPA1_APORTREQ (r) register accessor: an alias for `Reg<OPA1_APORTREQ_SPEC>`"]
pub type OPA1_APORTREQ = crate::Reg<opa1_aportreq::OPA1_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa1_aportreq;
#[doc = "OPA1_APORTCONFLICT (r) register accessor: an alias for `Reg<OPA1_APORTCONFLICT_SPEC>`"]
pub type OPA1_APORTCONFLICT = crate::Reg<opa1_aportconflict::OPA1_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa1_aportconflict;
#[doc = "OPA1_CTRL (rw) register accessor: an alias for `Reg<OPA1_CTRL_SPEC>`"]
pub type OPA1_CTRL = crate::Reg<opa1_ctrl::OPA1_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa1_ctrl;
#[doc = "OPA1_TIMER (rw) register accessor: an alias for `Reg<OPA1_TIMER_SPEC>`"]
pub type OPA1_TIMER = crate::Reg<opa1_timer::OPA1_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa1_timer;
#[doc = "OPA1_MUX (rw) register accessor: an alias for `Reg<OPA1_MUX_SPEC>`"]
pub type OPA1_MUX = crate::Reg<opa1_mux::OPA1_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1_mux;
#[doc = "OPA1_OUT (rw) register accessor: an alias for `Reg<OPA1_OUT_SPEC>`"]
pub type OPA1_OUT = crate::Reg<opa1_out::OPA1_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa1_out;
#[doc = "OPA1_CAL (rw) register accessor: an alias for `Reg<OPA1_CAL_SPEC>`"]
pub type OPA1_CAL = crate::Reg<opa1_cal::OPA1_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa1_cal;
#[doc = "OPA2_APORTREQ (r) register accessor: an alias for `Reg<OPA2_APORTREQ_SPEC>`"]
pub type OPA2_APORTREQ = crate::Reg<opa2_aportreq::OPA2_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa2_aportreq;
#[doc = "OPA2_APORTCONFLICT (r) register accessor: an alias for `Reg<OPA2_APORTCONFLICT_SPEC>`"]
pub type OPA2_APORTCONFLICT = crate::Reg<opa2_aportconflict::OPA2_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa2_aportconflict;
#[doc = "OPA2_CTRL (rw) register accessor: an alias for `Reg<OPA2_CTRL_SPEC>`"]
pub type OPA2_CTRL = crate::Reg<opa2_ctrl::OPA2_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa2_ctrl;
#[doc = "OPA2_TIMER (rw) register accessor: an alias for `Reg<OPA2_TIMER_SPEC>`"]
pub type OPA2_TIMER = crate::Reg<opa2_timer::OPA2_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa2_timer;
#[doc = "OPA2_MUX (rw) register accessor: an alias for `Reg<OPA2_MUX_SPEC>`"]
pub type OPA2_MUX = crate::Reg<opa2_mux::OPA2_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2_mux;
#[doc = "OPA2_OUT (rw) register accessor: an alias for `Reg<OPA2_OUT_SPEC>`"]
pub type OPA2_OUT = crate::Reg<opa2_out::OPA2_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa2_out;
#[doc = "OPA2_CAL (rw) register accessor: an alias for `Reg<OPA2_CAL_SPEC>`"]
pub type OPA2_CAL = crate::Reg<opa2_cal::OPA2_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa2_cal;
#[doc = "OPA3_APORTREQ (r) register accessor: an alias for `Reg<OPA3_APORTREQ_SPEC>`"]
pub type OPA3_APORTREQ = crate::Reg<opa3_aportreq::OPA3_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa3_aportreq;
#[doc = "OPA3_APORTCONFLICT (r) register accessor: an alias for `Reg<OPA3_APORTCONFLICT_SPEC>`"]
pub type OPA3_APORTCONFLICT = crate::Reg<opa3_aportconflict::OPA3_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa3_aportconflict;
#[doc = "OPA3_CTRL (rw) register accessor: an alias for `Reg<OPA3_CTRL_SPEC>`"]
pub type OPA3_CTRL = crate::Reg<opa3_ctrl::OPA3_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa3_ctrl;
#[doc = "OPA3_TIMER (rw) register accessor: an alias for `Reg<OPA3_TIMER_SPEC>`"]
pub type OPA3_TIMER = crate::Reg<opa3_timer::OPA3_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa3_timer;
#[doc = "OPA3_MUX (rw) register accessor: an alias for `Reg<OPA3_MUX_SPEC>`"]
pub type OPA3_MUX = crate::Reg<opa3_mux::OPA3_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa3_mux;
#[doc = "OPA3_OUT (rw) register accessor: an alias for `Reg<OPA3_OUT_SPEC>`"]
pub type OPA3_OUT = crate::Reg<opa3_out::OPA3_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa3_out;
#[doc = "OPA3_CAL (rw) register accessor: an alias for `Reg<OPA3_CAL_SPEC>`"]
pub type OPA3_CAL = crate::Reg<opa3_cal::OPA3_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa3_cal;
