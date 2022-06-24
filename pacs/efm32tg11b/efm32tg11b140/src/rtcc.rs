#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Pre-Counter Value Register"]
    pub precnt: crate::Reg<precnt::PRECNT_SPEC>,
    #[doc = "0x08 - Counter Value Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x0c - Combined Pre-Counter and Counter Value Register"]
    pub combcnt: crate::Reg<combcnt::COMBCNT_SPEC>,
    #[doc = "0x10 - Time of Day Register"]
    pub time: crate::Reg<time::TIME_SPEC>,
    #[doc = "0x14 - Date Register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0x18 - RTCC Interrupt Flags"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x28 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x2c - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x30 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x34 - Retention RAM Power-down Register"]
    pub powerdown: crate::Reg<powerdown::POWERDOWN_SPEC>,
    #[doc = "0x38 - Configuration Lock Register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x3c - Wake Up Enable"]
    pub em4wuen: crate::Reg<em4wuen::EM4WUEN_SPEC>,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc0_ctrl: crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>,
    #[doc = "0x44 - Capture/Compare Value Register"]
    pub cc0_ccv: crate::Reg<cc0_ccv::CC0_CCV_SPEC>,
    #[doc = "0x48 - Capture/Compare Time Register"]
    pub cc0_time: crate::Reg<cc0_time::CC0_TIME_SPEC>,
    #[doc = "0x4c - Capture/Compare Date Register"]
    pub cc0_date: crate::Reg<cc0_date::CC0_DATE_SPEC>,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc1_ctrl: crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>,
    #[doc = "0x54 - Capture/Compare Value Register"]
    pub cc1_ccv: crate::Reg<cc1_ccv::CC1_CCV_SPEC>,
    #[doc = "0x58 - Capture/Compare Time Register"]
    pub cc1_time: crate::Reg<cc1_time::CC1_TIME_SPEC>,
    #[doc = "0x5c - Capture/Compare Date Register"]
    pub cc1_date: crate::Reg<cc1_date::CC1_DATE_SPEC>,
    #[doc = "0x60 - CC Channel Control Register"]
    pub cc2_ctrl: crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>,
    #[doc = "0x64 - Capture/Compare Value Register"]
    pub cc2_ccv: crate::Reg<cc2_ccv::CC2_CCV_SPEC>,
    #[doc = "0x68 - Capture/Compare Time Register"]
    pub cc2_time: crate::Reg<cc2_time::CC2_TIME_SPEC>,
    #[doc = "0x6c - Capture/Compare Date Register"]
    pub cc2_date: crate::Reg<cc2_date::CC2_DATE_SPEC>,
    _reserved28: [u8; 0x94],
    #[doc = "0x104 - Retention Register"]
    pub ret0_reg: crate::Reg<ret0_reg::RET0_REG_SPEC>,
    #[doc = "0x108 - Retention Register"]
    pub ret1_reg: crate::Reg<ret1_reg::RET1_REG_SPEC>,
    #[doc = "0x10c - Retention Register"]
    pub ret2_reg: crate::Reg<ret2_reg::RET2_REG_SPEC>,
    #[doc = "0x110 - Retention Register"]
    pub ret3_reg: crate::Reg<ret3_reg::RET3_REG_SPEC>,
    #[doc = "0x114 - Retention Register"]
    pub ret4_reg: crate::Reg<ret4_reg::RET4_REG_SPEC>,
    #[doc = "0x118 - Retention Register"]
    pub ret5_reg: crate::Reg<ret5_reg::RET5_REG_SPEC>,
    #[doc = "0x11c - Retention Register"]
    pub ret6_reg: crate::Reg<ret6_reg::RET6_REG_SPEC>,
    #[doc = "0x120 - Retention Register"]
    pub ret7_reg: crate::Reg<ret7_reg::RET7_REG_SPEC>,
    #[doc = "0x124 - Retention Register"]
    pub ret8_reg: crate::Reg<ret8_reg::RET8_REG_SPEC>,
    #[doc = "0x128 - Retention Register"]
    pub ret9_reg: crate::Reg<ret9_reg::RET9_REG_SPEC>,
    #[doc = "0x12c - Retention Register"]
    pub ret10_reg: crate::Reg<ret10_reg::RET10_REG_SPEC>,
    #[doc = "0x130 - Retention Register"]
    pub ret11_reg: crate::Reg<ret11_reg::RET11_REG_SPEC>,
    #[doc = "0x134 - Retention Register"]
    pub ret12_reg: crate::Reg<ret12_reg::RET12_REG_SPEC>,
    #[doc = "0x138 - Retention Register"]
    pub ret13_reg: crate::Reg<ret13_reg::RET13_REG_SPEC>,
    #[doc = "0x13c - Retention Register"]
    pub ret14_reg: crate::Reg<ret14_reg::RET14_REG_SPEC>,
    #[doc = "0x140 - Retention Register"]
    pub ret15_reg: crate::Reg<ret15_reg::RET15_REG_SPEC>,
    #[doc = "0x144 - Retention Register"]
    pub ret16_reg: crate::Reg<ret16_reg::RET16_REG_SPEC>,
    #[doc = "0x148 - Retention Register"]
    pub ret17_reg: crate::Reg<ret17_reg::RET17_REG_SPEC>,
    #[doc = "0x14c - Retention Register"]
    pub ret18_reg: crate::Reg<ret18_reg::RET18_REG_SPEC>,
    #[doc = "0x150 - Retention Register"]
    pub ret19_reg: crate::Reg<ret19_reg::RET19_REG_SPEC>,
    #[doc = "0x154 - Retention Register"]
    pub ret20_reg: crate::Reg<ret20_reg::RET20_REG_SPEC>,
    #[doc = "0x158 - Retention Register"]
    pub ret21_reg: crate::Reg<ret21_reg::RET21_REG_SPEC>,
    #[doc = "0x15c - Retention Register"]
    pub ret22_reg: crate::Reg<ret22_reg::RET22_REG_SPEC>,
    #[doc = "0x160 - Retention Register"]
    pub ret23_reg: crate::Reg<ret23_reg::RET23_REG_SPEC>,
    #[doc = "0x164 - Retention Register"]
    pub ret24_reg: crate::Reg<ret24_reg::RET24_REG_SPEC>,
    #[doc = "0x168 - Retention Register"]
    pub ret25_reg: crate::Reg<ret25_reg::RET25_REG_SPEC>,
    #[doc = "0x16c - Retention Register"]
    pub ret26_reg: crate::Reg<ret26_reg::RET26_REG_SPEC>,
    #[doc = "0x170 - Retention Register"]
    pub ret27_reg: crate::Reg<ret27_reg::RET27_REG_SPEC>,
    #[doc = "0x174 - Retention Register"]
    pub ret28_reg: crate::Reg<ret28_reg::RET28_REG_SPEC>,
    #[doc = "0x178 - Retention Register"]
    pub ret29_reg: crate::Reg<ret29_reg::RET29_REG_SPEC>,
    #[doc = "0x17c - Retention Register"]
    pub ret30_reg: crate::Reg<ret30_reg::RET30_REG_SPEC>,
    #[doc = "0x180 - Retention Register"]
    pub ret31_reg: crate::Reg<ret31_reg::RET31_REG_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "PRECNT register accessor: an alias for `Reg<PRECNT_SPEC>`"]
pub type PRECNT = crate::Reg<precnt::PRECNT_SPEC>;
#[doc = "Pre-Counter Value Register"]
pub mod precnt;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "COMBCNT register accessor: an alias for `Reg<COMBCNT_SPEC>`"]
pub type COMBCNT = crate::Reg<combcnt::COMBCNT_SPEC>;
#[doc = "Combined Pre-Counter and Counter Value Register"]
pub mod combcnt;
#[doc = "TIME register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Time of Day Register"]
pub mod time;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date Register"]
pub mod date;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "RTCC Interrupt Flags"]
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
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "POWERDOWN register accessor: an alias for `Reg<POWERDOWN_SPEC>`"]
pub type POWERDOWN = crate::Reg<powerdown::POWERDOWN_SPEC>;
#[doc = "Retention RAM Power-down Register"]
pub mod powerdown;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "EM4WUEN register accessor: an alias for `Reg<EM4WUEN_SPEC>`"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "Wake Up Enable"]
pub mod em4wuen;
#[doc = "CC0_CTRL register accessor: an alias for `Reg<CC0_CTRL_SPEC>`"]
pub type CC0_CTRL = crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC0_CCV register accessor: an alias for `Reg<CC0_CCV_SPEC>`"]
pub type CC0_CCV = crate::Reg<cc0_ccv::CC0_CCV_SPEC>;
#[doc = "Capture/Compare Value Register"]
pub mod cc0_ccv;
#[doc = "CC0_TIME register accessor: an alias for `Reg<CC0_TIME_SPEC>`"]
pub type CC0_TIME = crate::Reg<cc0_time::CC0_TIME_SPEC>;
#[doc = "Capture/Compare Time Register"]
pub mod cc0_time;
#[doc = "CC0_DATE register accessor: an alias for `Reg<CC0_DATE_SPEC>`"]
pub type CC0_DATE = crate::Reg<cc0_date::CC0_DATE_SPEC>;
#[doc = "Capture/Compare Date Register"]
pub mod cc0_date;
#[doc = "CC1_CTRL register accessor: an alias for `Reg<CC1_CTRL_SPEC>`"]
pub type CC1_CTRL = crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC1_CCV register accessor: an alias for `Reg<CC1_CCV_SPEC>`"]
pub type CC1_CCV = crate::Reg<cc1_ccv::CC1_CCV_SPEC>;
#[doc = "Capture/Compare Value Register"]
pub mod cc1_ccv;
#[doc = "CC1_TIME register accessor: an alias for `Reg<CC1_TIME_SPEC>`"]
pub type CC1_TIME = crate::Reg<cc1_time::CC1_TIME_SPEC>;
#[doc = "Capture/Compare Time Register"]
pub mod cc1_time;
#[doc = "CC1_DATE register accessor: an alias for `Reg<CC1_DATE_SPEC>`"]
pub type CC1_DATE = crate::Reg<cc1_date::CC1_DATE_SPEC>;
#[doc = "Capture/Compare Date Register"]
pub mod cc1_date;
#[doc = "CC2_CTRL register accessor: an alias for `Reg<CC2_CTRL_SPEC>`"]
pub type CC2_CTRL = crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC2_CCV register accessor: an alias for `Reg<CC2_CCV_SPEC>`"]
pub type CC2_CCV = crate::Reg<cc2_ccv::CC2_CCV_SPEC>;
#[doc = "Capture/Compare Value Register"]
pub mod cc2_ccv;
#[doc = "CC2_TIME register accessor: an alias for `Reg<CC2_TIME_SPEC>`"]
pub type CC2_TIME = crate::Reg<cc2_time::CC2_TIME_SPEC>;
#[doc = "Capture/Compare Time Register"]
pub mod cc2_time;
#[doc = "CC2_DATE register accessor: an alias for `Reg<CC2_DATE_SPEC>`"]
pub type CC2_DATE = crate::Reg<cc2_date::CC2_DATE_SPEC>;
#[doc = "Capture/Compare Date Register"]
pub mod cc2_date;
#[doc = "RET0_REG register accessor: an alias for `Reg<RET0_REG_SPEC>`"]
pub type RET0_REG = crate::Reg<ret0_reg::RET0_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "RET1_REG register accessor: an alias for `Reg<RET1_REG_SPEC>`"]
pub type RET1_REG = crate::Reg<ret1_reg::RET1_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "RET2_REG register accessor: an alias for `Reg<RET2_REG_SPEC>`"]
pub type RET2_REG = crate::Reg<ret2_reg::RET2_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "RET3_REG register accessor: an alias for `Reg<RET3_REG_SPEC>`"]
pub type RET3_REG = crate::Reg<ret3_reg::RET3_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "RET4_REG register accessor: an alias for `Reg<RET4_REG_SPEC>`"]
pub type RET4_REG = crate::Reg<ret4_reg::RET4_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "RET5_REG register accessor: an alias for `Reg<RET5_REG_SPEC>`"]
pub type RET5_REG = crate::Reg<ret5_reg::RET5_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "RET6_REG register accessor: an alias for `Reg<RET6_REG_SPEC>`"]
pub type RET6_REG = crate::Reg<ret6_reg::RET6_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "RET7_REG register accessor: an alias for `Reg<RET7_REG_SPEC>`"]
pub type RET7_REG = crate::Reg<ret7_reg::RET7_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "RET8_REG register accessor: an alias for `Reg<RET8_REG_SPEC>`"]
pub type RET8_REG = crate::Reg<ret8_reg::RET8_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "RET9_REG register accessor: an alias for `Reg<RET9_REG_SPEC>`"]
pub type RET9_REG = crate::Reg<ret9_reg::RET9_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "RET10_REG register accessor: an alias for `Reg<RET10_REG_SPEC>`"]
pub type RET10_REG = crate::Reg<ret10_reg::RET10_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "RET11_REG register accessor: an alias for `Reg<RET11_REG_SPEC>`"]
pub type RET11_REG = crate::Reg<ret11_reg::RET11_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "RET12_REG register accessor: an alias for `Reg<RET12_REG_SPEC>`"]
pub type RET12_REG = crate::Reg<ret12_reg::RET12_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "RET13_REG register accessor: an alias for `Reg<RET13_REG_SPEC>`"]
pub type RET13_REG = crate::Reg<ret13_reg::RET13_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "RET14_REG register accessor: an alias for `Reg<RET14_REG_SPEC>`"]
pub type RET14_REG = crate::Reg<ret14_reg::RET14_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "RET15_REG register accessor: an alias for `Reg<RET15_REG_SPEC>`"]
pub type RET15_REG = crate::Reg<ret15_reg::RET15_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "RET16_REG register accessor: an alias for `Reg<RET16_REG_SPEC>`"]
pub type RET16_REG = crate::Reg<ret16_reg::RET16_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "RET17_REG register accessor: an alias for `Reg<RET17_REG_SPEC>`"]
pub type RET17_REG = crate::Reg<ret17_reg::RET17_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "RET18_REG register accessor: an alias for `Reg<RET18_REG_SPEC>`"]
pub type RET18_REG = crate::Reg<ret18_reg::RET18_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "RET19_REG register accessor: an alias for `Reg<RET19_REG_SPEC>`"]
pub type RET19_REG = crate::Reg<ret19_reg::RET19_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "RET20_REG register accessor: an alias for `Reg<RET20_REG_SPEC>`"]
pub type RET20_REG = crate::Reg<ret20_reg::RET20_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "RET21_REG register accessor: an alias for `Reg<RET21_REG_SPEC>`"]
pub type RET21_REG = crate::Reg<ret21_reg::RET21_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "RET22_REG register accessor: an alias for `Reg<RET22_REG_SPEC>`"]
pub type RET22_REG = crate::Reg<ret22_reg::RET22_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "RET23_REG register accessor: an alias for `Reg<RET23_REG_SPEC>`"]
pub type RET23_REG = crate::Reg<ret23_reg::RET23_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "RET24_REG register accessor: an alias for `Reg<RET24_REG_SPEC>`"]
pub type RET24_REG = crate::Reg<ret24_reg::RET24_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "RET25_REG register accessor: an alias for `Reg<RET25_REG_SPEC>`"]
pub type RET25_REG = crate::Reg<ret25_reg::RET25_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "RET26_REG register accessor: an alias for `Reg<RET26_REG_SPEC>`"]
pub type RET26_REG = crate::Reg<ret26_reg::RET26_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "RET27_REG register accessor: an alias for `Reg<RET27_REG_SPEC>`"]
pub type RET27_REG = crate::Reg<ret27_reg::RET27_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "RET28_REG register accessor: an alias for `Reg<RET28_REG_SPEC>`"]
pub type RET28_REG = crate::Reg<ret28_reg::RET28_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "RET29_REG register accessor: an alias for `Reg<RET29_REG_SPEC>`"]
pub type RET29_REG = crate::Reg<ret29_reg::RET29_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "RET30_REG register accessor: an alias for `Reg<RET30_REG_SPEC>`"]
pub type RET30_REG = crate::Reg<ret30_reg::RET30_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "RET31_REG register accessor: an alias for `Reg<RET31_REG_SPEC>`"]
pub type RET31_REG = crate::Reg<ret31_reg::RET31_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret31_reg;
