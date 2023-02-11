#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Timing Control Register"]
    pub timctrl: TIMCTRL,
    #[doc = "0x08 - Peripheral Control Register"]
    pub perctrl: PERCTRL,
    #[doc = "0x0c - Decoder Control Register"]
    pub decctrl: DECCTRL,
    #[doc = "0x10 - Bias Control Register"]
    pub biasctrl: BIASCTRL,
    #[doc = "0x14 - LESENSE Evaluation Control"]
    pub evalctrl: EVALCTRL,
    #[doc = "0x18 - PRS Control Register"]
    pub prsctrl: PRSCTRL,
    #[doc = "0x1c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x20 - Channel Enable Register"]
    pub chen: CHEN,
    #[doc = "0x24 - Scan Result Register"]
    pub scanres: SCANRES,
    #[doc = "0x28 - Status Register"]
    pub status: STATUS,
    #[doc = "0x2c - Result Buffer Pointers"]
    pub ptr: PTR,
    #[doc = "0x30 - Result Buffer Data Register"]
    pub bufdata: BUFDATA,
    #[doc = "0x34 - Current Channel Index"]
    pub curch: CURCH,
    #[doc = "0x38 - Current Decoder State"]
    pub decstate: DECSTATE,
    #[doc = "0x3c - Decoder Input Register"]
    pub sensorstate: SENSORSTATE,
    #[doc = "0x40 - GPIO Idle Phase Configuration"]
    pub idleconf: IDLECONF,
    #[doc = "0x44 - Alternative Excite Pin Configuration"]
    pub altexconf: ALTEXCONF,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x54 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x58 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x5c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x60 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x64 - I/O Routing Register"]
    pub routepen: ROUTEPEN,
    _reserved24: [u8; 0x98],
    #[doc = "0x100 - State Transition Configuration a"]
    pub st0_tconfa: ST0_TCONFA,
    #[doc = "0x104 - State Transition Configuration B"]
    pub st0_tconfb: ST0_TCONFB,
    #[doc = "0x108 - State Transition Configuration a"]
    pub st1_tconfa: ST1_TCONFA,
    #[doc = "0x10c - State Transition Configuration B"]
    pub st1_tconfb: ST1_TCONFB,
    #[doc = "0x110 - State Transition Configuration a"]
    pub st2_tconfa: ST2_TCONFA,
    #[doc = "0x114 - State Transition Configuration B"]
    pub st2_tconfb: ST2_TCONFB,
    #[doc = "0x118 - State Transition Configuration a"]
    pub st3_tconfa: ST3_TCONFA,
    #[doc = "0x11c - State Transition Configuration B"]
    pub st3_tconfb: ST3_TCONFB,
    #[doc = "0x120 - State Transition Configuration a"]
    pub st4_tconfa: ST4_TCONFA,
    #[doc = "0x124 - State Transition Configuration B"]
    pub st4_tconfb: ST4_TCONFB,
    #[doc = "0x128 - State Transition Configuration a"]
    pub st5_tconfa: ST5_TCONFA,
    #[doc = "0x12c - State Transition Configuration B"]
    pub st5_tconfb: ST5_TCONFB,
    #[doc = "0x130 - State Transition Configuration a"]
    pub st6_tconfa: ST6_TCONFA,
    #[doc = "0x134 - State Transition Configuration B"]
    pub st6_tconfb: ST6_TCONFB,
    #[doc = "0x138 - State Transition Configuration a"]
    pub st7_tconfa: ST7_TCONFA,
    #[doc = "0x13c - State Transition Configuration B"]
    pub st7_tconfb: ST7_TCONFB,
    #[doc = "0x140 - State Transition Configuration a"]
    pub st8_tconfa: ST8_TCONFA,
    #[doc = "0x144 - State Transition Configuration B"]
    pub st8_tconfb: ST8_TCONFB,
    #[doc = "0x148 - State Transition Configuration a"]
    pub st9_tconfa: ST9_TCONFA,
    #[doc = "0x14c - State Transition Configuration B"]
    pub st9_tconfb: ST9_TCONFB,
    #[doc = "0x150 - State Transition Configuration a"]
    pub st10_tconfa: ST10_TCONFA,
    #[doc = "0x154 - State Transition Configuration B"]
    pub st10_tconfb: ST10_TCONFB,
    #[doc = "0x158 - State Transition Configuration a"]
    pub st11_tconfa: ST11_TCONFA,
    #[doc = "0x15c - State Transition Configuration B"]
    pub st11_tconfb: ST11_TCONFB,
    #[doc = "0x160 - State Transition Configuration a"]
    pub st12_tconfa: ST12_TCONFA,
    #[doc = "0x164 - State Transition Configuration B"]
    pub st12_tconfb: ST12_TCONFB,
    #[doc = "0x168 - State Transition Configuration a"]
    pub st13_tconfa: ST13_TCONFA,
    #[doc = "0x16c - State Transition Configuration B"]
    pub st13_tconfb: ST13_TCONFB,
    #[doc = "0x170 - State Transition Configuration a"]
    pub st14_tconfa: ST14_TCONFA,
    #[doc = "0x174 - State Transition Configuration B"]
    pub st14_tconfb: ST14_TCONFB,
    #[doc = "0x178 - State Transition Configuration a"]
    pub st15_tconfa: ST15_TCONFA,
    #[doc = "0x17c - State Transition Configuration B"]
    pub st15_tconfb: ST15_TCONFB,
    #[doc = "0x180 - State Transition Configuration a"]
    pub st16_tconfa: ST16_TCONFA,
    #[doc = "0x184 - State Transition Configuration B"]
    pub st16_tconfb: ST16_TCONFB,
    #[doc = "0x188 - State Transition Configuration a"]
    pub st17_tconfa: ST17_TCONFA,
    #[doc = "0x18c - State Transition Configuration B"]
    pub st17_tconfb: ST17_TCONFB,
    #[doc = "0x190 - State Transition Configuration a"]
    pub st18_tconfa: ST18_TCONFA,
    #[doc = "0x194 - State Transition Configuration B"]
    pub st18_tconfb: ST18_TCONFB,
    #[doc = "0x198 - State Transition Configuration a"]
    pub st19_tconfa: ST19_TCONFA,
    #[doc = "0x19c - State Transition Configuration B"]
    pub st19_tconfb: ST19_TCONFB,
    #[doc = "0x1a0 - State Transition Configuration a"]
    pub st20_tconfa: ST20_TCONFA,
    #[doc = "0x1a4 - State Transition Configuration B"]
    pub st20_tconfb: ST20_TCONFB,
    #[doc = "0x1a8 - State Transition Configuration a"]
    pub st21_tconfa: ST21_TCONFA,
    #[doc = "0x1ac - State Transition Configuration B"]
    pub st21_tconfb: ST21_TCONFB,
    #[doc = "0x1b0 - State Transition Configuration a"]
    pub st22_tconfa: ST22_TCONFA,
    #[doc = "0x1b4 - State Transition Configuration B"]
    pub st22_tconfb: ST22_TCONFB,
    #[doc = "0x1b8 - State Transition Configuration a"]
    pub st23_tconfa: ST23_TCONFA,
    #[doc = "0x1bc - State Transition Configuration B"]
    pub st23_tconfb: ST23_TCONFB,
    #[doc = "0x1c0 - State Transition Configuration a"]
    pub st24_tconfa: ST24_TCONFA,
    #[doc = "0x1c4 - State Transition Configuration B"]
    pub st24_tconfb: ST24_TCONFB,
    #[doc = "0x1c8 - State Transition Configuration a"]
    pub st25_tconfa: ST25_TCONFA,
    #[doc = "0x1cc - State Transition Configuration B"]
    pub st25_tconfb: ST25_TCONFB,
    #[doc = "0x1d0 - State Transition Configuration a"]
    pub st26_tconfa: ST26_TCONFA,
    #[doc = "0x1d4 - State Transition Configuration B"]
    pub st26_tconfb: ST26_TCONFB,
    #[doc = "0x1d8 - State Transition Configuration a"]
    pub st27_tconfa: ST27_TCONFA,
    #[doc = "0x1dc - State Transition Configuration B"]
    pub st27_tconfb: ST27_TCONFB,
    #[doc = "0x1e0 - State Transition Configuration a"]
    pub st28_tconfa: ST28_TCONFA,
    #[doc = "0x1e4 - State Transition Configuration B"]
    pub st28_tconfb: ST28_TCONFB,
    #[doc = "0x1e8 - State Transition Configuration a"]
    pub st29_tconfa: ST29_TCONFA,
    #[doc = "0x1ec - State Transition Configuration B"]
    pub st29_tconfb: ST29_TCONFB,
    #[doc = "0x1f0 - State Transition Configuration a"]
    pub st30_tconfa: ST30_TCONFA,
    #[doc = "0x1f4 - State Transition Configuration B"]
    pub st30_tconfb: ST30_TCONFB,
    #[doc = "0x1f8 - State Transition Configuration a"]
    pub st31_tconfa: ST31_TCONFA,
    #[doc = "0x1fc - State Transition Configuration B"]
    pub st31_tconfb: ST31_TCONFB,
    #[doc = "0x200 - Scan Results"]
    pub buf0_data: BUF0_DATA,
    #[doc = "0x204 - Scan Results"]
    pub buf1_data: BUF1_DATA,
    #[doc = "0x208 - Scan Results"]
    pub buf2_data: BUF2_DATA,
    #[doc = "0x20c - Scan Results"]
    pub buf3_data: BUF3_DATA,
    #[doc = "0x210 - Scan Results"]
    pub buf4_data: BUF4_DATA,
    #[doc = "0x214 - Scan Results"]
    pub buf5_data: BUF5_DATA,
    #[doc = "0x218 - Scan Results"]
    pub buf6_data: BUF6_DATA,
    #[doc = "0x21c - Scan Results"]
    pub buf7_data: BUF7_DATA,
    #[doc = "0x220 - Scan Results"]
    pub buf8_data: BUF8_DATA,
    #[doc = "0x224 - Scan Results"]
    pub buf9_data: BUF9_DATA,
    #[doc = "0x228 - Scan Results"]
    pub buf10_data: BUF10_DATA,
    #[doc = "0x22c - Scan Results"]
    pub buf11_data: BUF11_DATA,
    #[doc = "0x230 - Scan Results"]
    pub buf12_data: BUF12_DATA,
    #[doc = "0x234 - Scan Results"]
    pub buf13_data: BUF13_DATA,
    #[doc = "0x238 - Scan Results"]
    pub buf14_data: BUF14_DATA,
    #[doc = "0x23c - Scan Results"]
    pub buf15_data: BUF15_DATA,
    #[doc = "0x240 - Scan Configuration"]
    pub ch0_timing: CH0_TIMING,
    #[doc = "0x244 - Scan Configuration"]
    pub ch0_interact: CH0_INTERACT,
    #[doc = "0x248 - Scan Configuration"]
    pub ch0_eval: CH0_EVAL,
    _reserved107: [u8; 0x04],
    #[doc = "0x250 - Scan Configuration"]
    pub ch1_timing: CH1_TIMING,
    #[doc = "0x254 - Scan Configuration"]
    pub ch1_interact: CH1_INTERACT,
    #[doc = "0x258 - Scan Configuration"]
    pub ch1_eval: CH1_EVAL,
    _reserved110: [u8; 0x04],
    #[doc = "0x260 - Scan Configuration"]
    pub ch2_timing: CH2_TIMING,
    #[doc = "0x264 - Scan Configuration"]
    pub ch2_interact: CH2_INTERACT,
    #[doc = "0x268 - Scan Configuration"]
    pub ch2_eval: CH2_EVAL,
    _reserved113: [u8; 0x04],
    #[doc = "0x270 - Scan Configuration"]
    pub ch3_timing: CH3_TIMING,
    #[doc = "0x274 - Scan Configuration"]
    pub ch3_interact: CH3_INTERACT,
    #[doc = "0x278 - Scan Configuration"]
    pub ch3_eval: CH3_EVAL,
    _reserved116: [u8; 0x04],
    #[doc = "0x280 - Scan Configuration"]
    pub ch4_timing: CH4_TIMING,
    #[doc = "0x284 - Scan Configuration"]
    pub ch4_interact: CH4_INTERACT,
    #[doc = "0x288 - Scan Configuration"]
    pub ch4_eval: CH4_EVAL,
    _reserved119: [u8; 0x04],
    #[doc = "0x290 - Scan Configuration"]
    pub ch5_timing: CH5_TIMING,
    #[doc = "0x294 - Scan Configuration"]
    pub ch5_interact: CH5_INTERACT,
    #[doc = "0x298 - Scan Configuration"]
    pub ch5_eval: CH5_EVAL,
    _reserved122: [u8; 0x04],
    #[doc = "0x2a0 - Scan Configuration"]
    pub ch6_timing: CH6_TIMING,
    #[doc = "0x2a4 - Scan Configuration"]
    pub ch6_interact: CH6_INTERACT,
    #[doc = "0x2a8 - Scan Configuration"]
    pub ch6_eval: CH6_EVAL,
    _reserved125: [u8; 0x04],
    #[doc = "0x2b0 - Scan Configuration"]
    pub ch7_timing: CH7_TIMING,
    #[doc = "0x2b4 - Scan Configuration"]
    pub ch7_interact: CH7_INTERACT,
    #[doc = "0x2b8 - Scan Configuration"]
    pub ch7_eval: CH7_EVAL,
    _reserved128: [u8; 0x04],
    #[doc = "0x2c0 - Scan Configuration"]
    pub ch8_timing: CH8_TIMING,
    #[doc = "0x2c4 - Scan Configuration"]
    pub ch8_interact: CH8_INTERACT,
    #[doc = "0x2c8 - Scan Configuration"]
    pub ch8_eval: CH8_EVAL,
    _reserved131: [u8; 0x04],
    #[doc = "0x2d0 - Scan Configuration"]
    pub ch9_timing: CH9_TIMING,
    #[doc = "0x2d4 - Scan Configuration"]
    pub ch9_interact: CH9_INTERACT,
    #[doc = "0x2d8 - Scan Configuration"]
    pub ch9_eval: CH9_EVAL,
    _reserved134: [u8; 0x04],
    #[doc = "0x2e0 - Scan Configuration"]
    pub ch10_timing: CH10_TIMING,
    #[doc = "0x2e4 - Scan Configuration"]
    pub ch10_interact: CH10_INTERACT,
    #[doc = "0x2e8 - Scan Configuration"]
    pub ch10_eval: CH10_EVAL,
    _reserved137: [u8; 0x04],
    #[doc = "0x2f0 - Scan Configuration"]
    pub ch11_timing: CH11_TIMING,
    #[doc = "0x2f4 - Scan Configuration"]
    pub ch11_interact: CH11_INTERACT,
    #[doc = "0x2f8 - Scan Configuration"]
    pub ch11_eval: CH11_EVAL,
    _reserved140: [u8; 0x04],
    #[doc = "0x300 - Scan Configuration"]
    pub ch12_timing: CH12_TIMING,
    #[doc = "0x304 - Scan Configuration"]
    pub ch12_interact: CH12_INTERACT,
    #[doc = "0x308 - Scan Configuration"]
    pub ch12_eval: CH12_EVAL,
    _reserved143: [u8; 0x04],
    #[doc = "0x310 - Scan Configuration"]
    pub ch13_timing: CH13_TIMING,
    #[doc = "0x314 - Scan Configuration"]
    pub ch13_interact: CH13_INTERACT,
    #[doc = "0x318 - Scan Configuration"]
    pub ch13_eval: CH13_EVAL,
    _reserved146: [u8; 0x04],
    #[doc = "0x320 - Scan Configuration"]
    pub ch14_timing: CH14_TIMING,
    #[doc = "0x324 - Scan Configuration"]
    pub ch14_interact: CH14_INTERACT,
    #[doc = "0x328 - Scan Configuration"]
    pub ch14_eval: CH14_EVAL,
    _reserved149: [u8; 0x04],
    #[doc = "0x330 - Scan Configuration"]
    pub ch15_timing: CH15_TIMING,
    #[doc = "0x334 - Scan Configuration"]
    pub ch15_interact: CH15_INTERACT,
    #[doc = "0x338 - Scan Configuration"]
    pub ch15_eval: CH15_EVAL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "TIMCTRL (rw) register accessor: an alias for `Reg<TIMCTRL_SPEC>`"]
pub type TIMCTRL = crate::Reg<timctrl::TIMCTRL_SPEC>;
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "PERCTRL (rw) register accessor: an alias for `Reg<PERCTRL_SPEC>`"]
pub type PERCTRL = crate::Reg<perctrl::PERCTRL_SPEC>;
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "DECCTRL (rw) register accessor: an alias for `Reg<DECCTRL_SPEC>`"]
pub type DECCTRL = crate::Reg<decctrl::DECCTRL_SPEC>;
#[doc = "Decoder Control Register"]
pub mod decctrl;
#[doc = "BIASCTRL (rw) register accessor: an alias for `Reg<BIASCTRL_SPEC>`"]
pub type BIASCTRL = crate::Reg<biasctrl::BIASCTRL_SPEC>;
#[doc = "Bias Control Register"]
pub mod biasctrl;
#[doc = "EVALCTRL (rw) register accessor: an alias for `Reg<EVALCTRL_SPEC>`"]
pub type EVALCTRL = crate::Reg<evalctrl::EVALCTRL_SPEC>;
#[doc = "LESENSE Evaluation Control"]
pub mod evalctrl;
#[doc = "PRSCTRL (rw) register accessor: an alias for `Reg<PRSCTRL_SPEC>`"]
pub type PRSCTRL = crate::Reg<prsctrl::PRSCTRL_SPEC>;
#[doc = "PRS Control Register"]
pub mod prsctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CHEN (rw) register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "SCANRES (rw) register accessor: an alias for `Reg<SCANRES_SPEC>`"]
pub type SCANRES = crate::Reg<scanres::SCANRES_SPEC>;
#[doc = "Scan Result Register"]
pub mod scanres;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "PTR (r) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Result Buffer Pointers"]
pub mod ptr;
#[doc = "BUFDATA (r) register accessor: an alias for `Reg<BUFDATA_SPEC>`"]
pub type BUFDATA = crate::Reg<bufdata::BUFDATA_SPEC>;
#[doc = "Result Buffer Data Register"]
pub mod bufdata;
#[doc = "CURCH (r) register accessor: an alias for `Reg<CURCH_SPEC>`"]
pub type CURCH = crate::Reg<curch::CURCH_SPEC>;
#[doc = "Current Channel Index"]
pub mod curch;
#[doc = "DECSTATE (rw) register accessor: an alias for `Reg<DECSTATE_SPEC>`"]
pub type DECSTATE = crate::Reg<decstate::DECSTATE_SPEC>;
#[doc = "Current Decoder State"]
pub mod decstate;
#[doc = "SENSORSTATE (rw) register accessor: an alias for `Reg<SENSORSTATE_SPEC>`"]
pub type SENSORSTATE = crate::Reg<sensorstate::SENSORSTATE_SPEC>;
#[doc = "Decoder Input Register"]
pub mod sensorstate;
#[doc = "IDLECONF (rw) register accessor: an alias for `Reg<IDLECONF_SPEC>`"]
pub type IDLECONF = crate::Reg<idleconf::IDLECONF_SPEC>;
#[doc = "GPIO Idle Phase Configuration"]
pub mod idleconf;
#[doc = "ALTEXCONF (rw) register accessor: an alias for `Reg<ALTEXCONF_SPEC>`"]
pub type ALTEXCONF = crate::Reg<altexconf::ALTEXCONF_SPEC>;
#[doc = "Alternative Excite Pin Configuration"]
pub mod altexconf;
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
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "ST0_TCONFA (rw) register accessor: an alias for `Reg<ST0_TCONFA_SPEC>`"]
pub type ST0_TCONFA = crate::Reg<st0_tconfa::ST0_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st0_tconfa;
#[doc = "ST0_TCONFB (rw) register accessor: an alias for `Reg<ST0_TCONFB_SPEC>`"]
pub type ST0_TCONFB = crate::Reg<st0_tconfb::ST0_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st0_tconfb;
#[doc = "ST1_TCONFA (rw) register accessor: an alias for `Reg<ST1_TCONFA_SPEC>`"]
pub type ST1_TCONFA = crate::Reg<st1_tconfa::ST1_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st1_tconfa;
#[doc = "ST1_TCONFB (rw) register accessor: an alias for `Reg<ST1_TCONFB_SPEC>`"]
pub type ST1_TCONFB = crate::Reg<st1_tconfb::ST1_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st1_tconfb;
#[doc = "ST2_TCONFA (rw) register accessor: an alias for `Reg<ST2_TCONFA_SPEC>`"]
pub type ST2_TCONFA = crate::Reg<st2_tconfa::ST2_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st2_tconfa;
#[doc = "ST2_TCONFB (rw) register accessor: an alias for `Reg<ST2_TCONFB_SPEC>`"]
pub type ST2_TCONFB = crate::Reg<st2_tconfb::ST2_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st2_tconfb;
#[doc = "ST3_TCONFA (rw) register accessor: an alias for `Reg<ST3_TCONFA_SPEC>`"]
pub type ST3_TCONFA = crate::Reg<st3_tconfa::ST3_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st3_tconfa;
#[doc = "ST3_TCONFB (rw) register accessor: an alias for `Reg<ST3_TCONFB_SPEC>`"]
pub type ST3_TCONFB = crate::Reg<st3_tconfb::ST3_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st3_tconfb;
#[doc = "ST4_TCONFA (rw) register accessor: an alias for `Reg<ST4_TCONFA_SPEC>`"]
pub type ST4_TCONFA = crate::Reg<st4_tconfa::ST4_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st4_tconfa;
#[doc = "ST4_TCONFB (rw) register accessor: an alias for `Reg<ST4_TCONFB_SPEC>`"]
pub type ST4_TCONFB = crate::Reg<st4_tconfb::ST4_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st4_tconfb;
#[doc = "ST5_TCONFA (rw) register accessor: an alias for `Reg<ST5_TCONFA_SPEC>`"]
pub type ST5_TCONFA = crate::Reg<st5_tconfa::ST5_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st5_tconfa;
#[doc = "ST5_TCONFB (rw) register accessor: an alias for `Reg<ST5_TCONFB_SPEC>`"]
pub type ST5_TCONFB = crate::Reg<st5_tconfb::ST5_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st5_tconfb;
#[doc = "ST6_TCONFA (rw) register accessor: an alias for `Reg<ST6_TCONFA_SPEC>`"]
pub type ST6_TCONFA = crate::Reg<st6_tconfa::ST6_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st6_tconfa;
#[doc = "ST6_TCONFB (rw) register accessor: an alias for `Reg<ST6_TCONFB_SPEC>`"]
pub type ST6_TCONFB = crate::Reg<st6_tconfb::ST6_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st6_tconfb;
#[doc = "ST7_TCONFA (rw) register accessor: an alias for `Reg<ST7_TCONFA_SPEC>`"]
pub type ST7_TCONFA = crate::Reg<st7_tconfa::ST7_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st7_tconfa;
#[doc = "ST7_TCONFB (rw) register accessor: an alias for `Reg<ST7_TCONFB_SPEC>`"]
pub type ST7_TCONFB = crate::Reg<st7_tconfb::ST7_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st7_tconfb;
#[doc = "ST8_TCONFA (rw) register accessor: an alias for `Reg<ST8_TCONFA_SPEC>`"]
pub type ST8_TCONFA = crate::Reg<st8_tconfa::ST8_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st8_tconfa;
#[doc = "ST8_TCONFB (rw) register accessor: an alias for `Reg<ST8_TCONFB_SPEC>`"]
pub type ST8_TCONFB = crate::Reg<st8_tconfb::ST8_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st8_tconfb;
#[doc = "ST9_TCONFA (rw) register accessor: an alias for `Reg<ST9_TCONFA_SPEC>`"]
pub type ST9_TCONFA = crate::Reg<st9_tconfa::ST9_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st9_tconfa;
#[doc = "ST9_TCONFB (rw) register accessor: an alias for `Reg<ST9_TCONFB_SPEC>`"]
pub type ST9_TCONFB = crate::Reg<st9_tconfb::ST9_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st9_tconfb;
#[doc = "ST10_TCONFA (rw) register accessor: an alias for `Reg<ST10_TCONFA_SPEC>`"]
pub type ST10_TCONFA = crate::Reg<st10_tconfa::ST10_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st10_tconfa;
#[doc = "ST10_TCONFB (rw) register accessor: an alias for `Reg<ST10_TCONFB_SPEC>`"]
pub type ST10_TCONFB = crate::Reg<st10_tconfb::ST10_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st10_tconfb;
#[doc = "ST11_TCONFA (rw) register accessor: an alias for `Reg<ST11_TCONFA_SPEC>`"]
pub type ST11_TCONFA = crate::Reg<st11_tconfa::ST11_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st11_tconfa;
#[doc = "ST11_TCONFB (rw) register accessor: an alias for `Reg<ST11_TCONFB_SPEC>`"]
pub type ST11_TCONFB = crate::Reg<st11_tconfb::ST11_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st11_tconfb;
#[doc = "ST12_TCONFA (rw) register accessor: an alias for `Reg<ST12_TCONFA_SPEC>`"]
pub type ST12_TCONFA = crate::Reg<st12_tconfa::ST12_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st12_tconfa;
#[doc = "ST12_TCONFB (rw) register accessor: an alias for `Reg<ST12_TCONFB_SPEC>`"]
pub type ST12_TCONFB = crate::Reg<st12_tconfb::ST12_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st12_tconfb;
#[doc = "ST13_TCONFA (rw) register accessor: an alias for `Reg<ST13_TCONFA_SPEC>`"]
pub type ST13_TCONFA = crate::Reg<st13_tconfa::ST13_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st13_tconfa;
#[doc = "ST13_TCONFB (rw) register accessor: an alias for `Reg<ST13_TCONFB_SPEC>`"]
pub type ST13_TCONFB = crate::Reg<st13_tconfb::ST13_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st13_tconfb;
#[doc = "ST14_TCONFA (rw) register accessor: an alias for `Reg<ST14_TCONFA_SPEC>`"]
pub type ST14_TCONFA = crate::Reg<st14_tconfa::ST14_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st14_tconfa;
#[doc = "ST14_TCONFB (rw) register accessor: an alias for `Reg<ST14_TCONFB_SPEC>`"]
pub type ST14_TCONFB = crate::Reg<st14_tconfb::ST14_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st14_tconfb;
#[doc = "ST15_TCONFA (rw) register accessor: an alias for `Reg<ST15_TCONFA_SPEC>`"]
pub type ST15_TCONFA = crate::Reg<st15_tconfa::ST15_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st15_tconfa;
#[doc = "ST15_TCONFB (rw) register accessor: an alias for `Reg<ST15_TCONFB_SPEC>`"]
pub type ST15_TCONFB = crate::Reg<st15_tconfb::ST15_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st15_tconfb;
#[doc = "ST16_TCONFA (rw) register accessor: an alias for `Reg<ST16_TCONFA_SPEC>`"]
pub type ST16_TCONFA = crate::Reg<st16_tconfa::ST16_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st16_tconfa;
#[doc = "ST16_TCONFB (rw) register accessor: an alias for `Reg<ST16_TCONFB_SPEC>`"]
pub type ST16_TCONFB = crate::Reg<st16_tconfb::ST16_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st16_tconfb;
#[doc = "ST17_TCONFA (rw) register accessor: an alias for `Reg<ST17_TCONFA_SPEC>`"]
pub type ST17_TCONFA = crate::Reg<st17_tconfa::ST17_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st17_tconfa;
#[doc = "ST17_TCONFB (rw) register accessor: an alias for `Reg<ST17_TCONFB_SPEC>`"]
pub type ST17_TCONFB = crate::Reg<st17_tconfb::ST17_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st17_tconfb;
#[doc = "ST18_TCONFA (rw) register accessor: an alias for `Reg<ST18_TCONFA_SPEC>`"]
pub type ST18_TCONFA = crate::Reg<st18_tconfa::ST18_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st18_tconfa;
#[doc = "ST18_TCONFB (rw) register accessor: an alias for `Reg<ST18_TCONFB_SPEC>`"]
pub type ST18_TCONFB = crate::Reg<st18_tconfb::ST18_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st18_tconfb;
#[doc = "ST19_TCONFA (rw) register accessor: an alias for `Reg<ST19_TCONFA_SPEC>`"]
pub type ST19_TCONFA = crate::Reg<st19_tconfa::ST19_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st19_tconfa;
#[doc = "ST19_TCONFB (rw) register accessor: an alias for `Reg<ST19_TCONFB_SPEC>`"]
pub type ST19_TCONFB = crate::Reg<st19_tconfb::ST19_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st19_tconfb;
#[doc = "ST20_TCONFA (rw) register accessor: an alias for `Reg<ST20_TCONFA_SPEC>`"]
pub type ST20_TCONFA = crate::Reg<st20_tconfa::ST20_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st20_tconfa;
#[doc = "ST20_TCONFB (rw) register accessor: an alias for `Reg<ST20_TCONFB_SPEC>`"]
pub type ST20_TCONFB = crate::Reg<st20_tconfb::ST20_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st20_tconfb;
#[doc = "ST21_TCONFA (rw) register accessor: an alias for `Reg<ST21_TCONFA_SPEC>`"]
pub type ST21_TCONFA = crate::Reg<st21_tconfa::ST21_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st21_tconfa;
#[doc = "ST21_TCONFB (rw) register accessor: an alias for `Reg<ST21_TCONFB_SPEC>`"]
pub type ST21_TCONFB = crate::Reg<st21_tconfb::ST21_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st21_tconfb;
#[doc = "ST22_TCONFA (rw) register accessor: an alias for `Reg<ST22_TCONFA_SPEC>`"]
pub type ST22_TCONFA = crate::Reg<st22_tconfa::ST22_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st22_tconfa;
#[doc = "ST22_TCONFB (rw) register accessor: an alias for `Reg<ST22_TCONFB_SPEC>`"]
pub type ST22_TCONFB = crate::Reg<st22_tconfb::ST22_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st22_tconfb;
#[doc = "ST23_TCONFA (rw) register accessor: an alias for `Reg<ST23_TCONFA_SPEC>`"]
pub type ST23_TCONFA = crate::Reg<st23_tconfa::ST23_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st23_tconfa;
#[doc = "ST23_TCONFB (rw) register accessor: an alias for `Reg<ST23_TCONFB_SPEC>`"]
pub type ST23_TCONFB = crate::Reg<st23_tconfb::ST23_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st23_tconfb;
#[doc = "ST24_TCONFA (rw) register accessor: an alias for `Reg<ST24_TCONFA_SPEC>`"]
pub type ST24_TCONFA = crate::Reg<st24_tconfa::ST24_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st24_tconfa;
#[doc = "ST24_TCONFB (rw) register accessor: an alias for `Reg<ST24_TCONFB_SPEC>`"]
pub type ST24_TCONFB = crate::Reg<st24_tconfb::ST24_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st24_tconfb;
#[doc = "ST25_TCONFA (rw) register accessor: an alias for `Reg<ST25_TCONFA_SPEC>`"]
pub type ST25_TCONFA = crate::Reg<st25_tconfa::ST25_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st25_tconfa;
#[doc = "ST25_TCONFB (rw) register accessor: an alias for `Reg<ST25_TCONFB_SPEC>`"]
pub type ST25_TCONFB = crate::Reg<st25_tconfb::ST25_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st25_tconfb;
#[doc = "ST26_TCONFA (rw) register accessor: an alias for `Reg<ST26_TCONFA_SPEC>`"]
pub type ST26_TCONFA = crate::Reg<st26_tconfa::ST26_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st26_tconfa;
#[doc = "ST26_TCONFB (rw) register accessor: an alias for `Reg<ST26_TCONFB_SPEC>`"]
pub type ST26_TCONFB = crate::Reg<st26_tconfb::ST26_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st26_tconfb;
#[doc = "ST27_TCONFA (rw) register accessor: an alias for `Reg<ST27_TCONFA_SPEC>`"]
pub type ST27_TCONFA = crate::Reg<st27_tconfa::ST27_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st27_tconfa;
#[doc = "ST27_TCONFB (rw) register accessor: an alias for `Reg<ST27_TCONFB_SPEC>`"]
pub type ST27_TCONFB = crate::Reg<st27_tconfb::ST27_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st27_tconfb;
#[doc = "ST28_TCONFA (rw) register accessor: an alias for `Reg<ST28_TCONFA_SPEC>`"]
pub type ST28_TCONFA = crate::Reg<st28_tconfa::ST28_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st28_tconfa;
#[doc = "ST28_TCONFB (rw) register accessor: an alias for `Reg<ST28_TCONFB_SPEC>`"]
pub type ST28_TCONFB = crate::Reg<st28_tconfb::ST28_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st28_tconfb;
#[doc = "ST29_TCONFA (rw) register accessor: an alias for `Reg<ST29_TCONFA_SPEC>`"]
pub type ST29_TCONFA = crate::Reg<st29_tconfa::ST29_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st29_tconfa;
#[doc = "ST29_TCONFB (rw) register accessor: an alias for `Reg<ST29_TCONFB_SPEC>`"]
pub type ST29_TCONFB = crate::Reg<st29_tconfb::ST29_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st29_tconfb;
#[doc = "ST30_TCONFA (rw) register accessor: an alias for `Reg<ST30_TCONFA_SPEC>`"]
pub type ST30_TCONFA = crate::Reg<st30_tconfa::ST30_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st30_tconfa;
#[doc = "ST30_TCONFB (rw) register accessor: an alias for `Reg<ST30_TCONFB_SPEC>`"]
pub type ST30_TCONFB = crate::Reg<st30_tconfb::ST30_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st30_tconfb;
#[doc = "ST31_TCONFA (rw) register accessor: an alias for `Reg<ST31_TCONFA_SPEC>`"]
pub type ST31_TCONFA = crate::Reg<st31_tconfa::ST31_TCONFA_SPEC>;
#[doc = "State Transition Configuration a"]
pub mod st31_tconfa;
#[doc = "ST31_TCONFB (rw) register accessor: an alias for `Reg<ST31_TCONFB_SPEC>`"]
pub type ST31_TCONFB = crate::Reg<st31_tconfb::ST31_TCONFB_SPEC>;
#[doc = "State Transition Configuration B"]
pub mod st31_tconfb;
#[doc = "BUF0_DATA (rw) register accessor: an alias for `Reg<BUF0_DATA_SPEC>`"]
pub type BUF0_DATA = crate::Reg<buf0_data::BUF0_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf0_data;
#[doc = "BUF1_DATA (rw) register accessor: an alias for `Reg<BUF1_DATA_SPEC>`"]
pub type BUF1_DATA = crate::Reg<buf1_data::BUF1_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf1_data;
#[doc = "BUF2_DATA (rw) register accessor: an alias for `Reg<BUF2_DATA_SPEC>`"]
pub type BUF2_DATA = crate::Reg<buf2_data::BUF2_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf2_data;
#[doc = "BUF3_DATA (rw) register accessor: an alias for `Reg<BUF3_DATA_SPEC>`"]
pub type BUF3_DATA = crate::Reg<buf3_data::BUF3_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf3_data;
#[doc = "BUF4_DATA (rw) register accessor: an alias for `Reg<BUF4_DATA_SPEC>`"]
pub type BUF4_DATA = crate::Reg<buf4_data::BUF4_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf4_data;
#[doc = "BUF5_DATA (rw) register accessor: an alias for `Reg<BUF5_DATA_SPEC>`"]
pub type BUF5_DATA = crate::Reg<buf5_data::BUF5_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf5_data;
#[doc = "BUF6_DATA (rw) register accessor: an alias for `Reg<BUF6_DATA_SPEC>`"]
pub type BUF6_DATA = crate::Reg<buf6_data::BUF6_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf6_data;
#[doc = "BUF7_DATA (rw) register accessor: an alias for `Reg<BUF7_DATA_SPEC>`"]
pub type BUF7_DATA = crate::Reg<buf7_data::BUF7_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf7_data;
#[doc = "BUF8_DATA (rw) register accessor: an alias for `Reg<BUF8_DATA_SPEC>`"]
pub type BUF8_DATA = crate::Reg<buf8_data::BUF8_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf8_data;
#[doc = "BUF9_DATA (rw) register accessor: an alias for `Reg<BUF9_DATA_SPEC>`"]
pub type BUF9_DATA = crate::Reg<buf9_data::BUF9_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf9_data;
#[doc = "BUF10_DATA (rw) register accessor: an alias for `Reg<BUF10_DATA_SPEC>`"]
pub type BUF10_DATA = crate::Reg<buf10_data::BUF10_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf10_data;
#[doc = "BUF11_DATA (rw) register accessor: an alias for `Reg<BUF11_DATA_SPEC>`"]
pub type BUF11_DATA = crate::Reg<buf11_data::BUF11_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf11_data;
#[doc = "BUF12_DATA (rw) register accessor: an alias for `Reg<BUF12_DATA_SPEC>`"]
pub type BUF12_DATA = crate::Reg<buf12_data::BUF12_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf12_data;
#[doc = "BUF13_DATA (rw) register accessor: an alias for `Reg<BUF13_DATA_SPEC>`"]
pub type BUF13_DATA = crate::Reg<buf13_data::BUF13_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf13_data;
#[doc = "BUF14_DATA (rw) register accessor: an alias for `Reg<BUF14_DATA_SPEC>`"]
pub type BUF14_DATA = crate::Reg<buf14_data::BUF14_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf14_data;
#[doc = "BUF15_DATA (rw) register accessor: an alias for `Reg<BUF15_DATA_SPEC>`"]
pub type BUF15_DATA = crate::Reg<buf15_data::BUF15_DATA_SPEC>;
#[doc = "Scan Results"]
pub mod buf15_data;
#[doc = "CH0_TIMING (rw) register accessor: an alias for `Reg<CH0_TIMING_SPEC>`"]
pub type CH0_TIMING = crate::Reg<ch0_timing::CH0_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch0_timing;
#[doc = "CH0_INTERACT (rw) register accessor: an alias for `Reg<CH0_INTERACT_SPEC>`"]
pub type CH0_INTERACT = crate::Reg<ch0_interact::CH0_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch0_interact;
#[doc = "CH0_EVAL (rw) register accessor: an alias for `Reg<CH0_EVAL_SPEC>`"]
pub type CH0_EVAL = crate::Reg<ch0_eval::CH0_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch0_eval;
#[doc = "CH1_TIMING (rw) register accessor: an alias for `Reg<CH1_TIMING_SPEC>`"]
pub type CH1_TIMING = crate::Reg<ch1_timing::CH1_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch1_timing;
#[doc = "CH1_INTERACT (rw) register accessor: an alias for `Reg<CH1_INTERACT_SPEC>`"]
pub type CH1_INTERACT = crate::Reg<ch1_interact::CH1_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch1_interact;
#[doc = "CH1_EVAL (rw) register accessor: an alias for `Reg<CH1_EVAL_SPEC>`"]
pub type CH1_EVAL = crate::Reg<ch1_eval::CH1_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch1_eval;
#[doc = "CH2_TIMING (rw) register accessor: an alias for `Reg<CH2_TIMING_SPEC>`"]
pub type CH2_TIMING = crate::Reg<ch2_timing::CH2_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch2_timing;
#[doc = "CH2_INTERACT (rw) register accessor: an alias for `Reg<CH2_INTERACT_SPEC>`"]
pub type CH2_INTERACT = crate::Reg<ch2_interact::CH2_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch2_interact;
#[doc = "CH2_EVAL (rw) register accessor: an alias for `Reg<CH2_EVAL_SPEC>`"]
pub type CH2_EVAL = crate::Reg<ch2_eval::CH2_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch2_eval;
#[doc = "CH3_TIMING (rw) register accessor: an alias for `Reg<CH3_TIMING_SPEC>`"]
pub type CH3_TIMING = crate::Reg<ch3_timing::CH3_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch3_timing;
#[doc = "CH3_INTERACT (rw) register accessor: an alias for `Reg<CH3_INTERACT_SPEC>`"]
pub type CH3_INTERACT = crate::Reg<ch3_interact::CH3_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch3_interact;
#[doc = "CH3_EVAL (rw) register accessor: an alias for `Reg<CH3_EVAL_SPEC>`"]
pub type CH3_EVAL = crate::Reg<ch3_eval::CH3_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch3_eval;
#[doc = "CH4_TIMING (rw) register accessor: an alias for `Reg<CH4_TIMING_SPEC>`"]
pub type CH4_TIMING = crate::Reg<ch4_timing::CH4_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch4_timing;
#[doc = "CH4_INTERACT (rw) register accessor: an alias for `Reg<CH4_INTERACT_SPEC>`"]
pub type CH4_INTERACT = crate::Reg<ch4_interact::CH4_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch4_interact;
#[doc = "CH4_EVAL (rw) register accessor: an alias for `Reg<CH4_EVAL_SPEC>`"]
pub type CH4_EVAL = crate::Reg<ch4_eval::CH4_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch4_eval;
#[doc = "CH5_TIMING (rw) register accessor: an alias for `Reg<CH5_TIMING_SPEC>`"]
pub type CH5_TIMING = crate::Reg<ch5_timing::CH5_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch5_timing;
#[doc = "CH5_INTERACT (rw) register accessor: an alias for `Reg<CH5_INTERACT_SPEC>`"]
pub type CH5_INTERACT = crate::Reg<ch5_interact::CH5_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch5_interact;
#[doc = "CH5_EVAL (rw) register accessor: an alias for `Reg<CH5_EVAL_SPEC>`"]
pub type CH5_EVAL = crate::Reg<ch5_eval::CH5_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch5_eval;
#[doc = "CH6_TIMING (rw) register accessor: an alias for `Reg<CH6_TIMING_SPEC>`"]
pub type CH6_TIMING = crate::Reg<ch6_timing::CH6_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch6_timing;
#[doc = "CH6_INTERACT (rw) register accessor: an alias for `Reg<CH6_INTERACT_SPEC>`"]
pub type CH6_INTERACT = crate::Reg<ch6_interact::CH6_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch6_interact;
#[doc = "CH6_EVAL (rw) register accessor: an alias for `Reg<CH6_EVAL_SPEC>`"]
pub type CH6_EVAL = crate::Reg<ch6_eval::CH6_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch6_eval;
#[doc = "CH7_TIMING (rw) register accessor: an alias for `Reg<CH7_TIMING_SPEC>`"]
pub type CH7_TIMING = crate::Reg<ch7_timing::CH7_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch7_timing;
#[doc = "CH7_INTERACT (rw) register accessor: an alias for `Reg<CH7_INTERACT_SPEC>`"]
pub type CH7_INTERACT = crate::Reg<ch7_interact::CH7_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch7_interact;
#[doc = "CH7_EVAL (rw) register accessor: an alias for `Reg<CH7_EVAL_SPEC>`"]
pub type CH7_EVAL = crate::Reg<ch7_eval::CH7_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch7_eval;
#[doc = "CH8_TIMING (rw) register accessor: an alias for `Reg<CH8_TIMING_SPEC>`"]
pub type CH8_TIMING = crate::Reg<ch8_timing::CH8_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch8_timing;
#[doc = "CH8_INTERACT (rw) register accessor: an alias for `Reg<CH8_INTERACT_SPEC>`"]
pub type CH8_INTERACT = crate::Reg<ch8_interact::CH8_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch8_interact;
#[doc = "CH8_EVAL (rw) register accessor: an alias for `Reg<CH8_EVAL_SPEC>`"]
pub type CH8_EVAL = crate::Reg<ch8_eval::CH8_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch8_eval;
#[doc = "CH9_TIMING (rw) register accessor: an alias for `Reg<CH9_TIMING_SPEC>`"]
pub type CH9_TIMING = crate::Reg<ch9_timing::CH9_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch9_timing;
#[doc = "CH9_INTERACT (rw) register accessor: an alias for `Reg<CH9_INTERACT_SPEC>`"]
pub type CH9_INTERACT = crate::Reg<ch9_interact::CH9_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch9_interact;
#[doc = "CH9_EVAL (rw) register accessor: an alias for `Reg<CH9_EVAL_SPEC>`"]
pub type CH9_EVAL = crate::Reg<ch9_eval::CH9_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch9_eval;
#[doc = "CH10_TIMING (rw) register accessor: an alias for `Reg<CH10_TIMING_SPEC>`"]
pub type CH10_TIMING = crate::Reg<ch10_timing::CH10_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch10_timing;
#[doc = "CH10_INTERACT (rw) register accessor: an alias for `Reg<CH10_INTERACT_SPEC>`"]
pub type CH10_INTERACT = crate::Reg<ch10_interact::CH10_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch10_interact;
#[doc = "CH10_EVAL (rw) register accessor: an alias for `Reg<CH10_EVAL_SPEC>`"]
pub type CH10_EVAL = crate::Reg<ch10_eval::CH10_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch10_eval;
#[doc = "CH11_TIMING (rw) register accessor: an alias for `Reg<CH11_TIMING_SPEC>`"]
pub type CH11_TIMING = crate::Reg<ch11_timing::CH11_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch11_timing;
#[doc = "CH11_INTERACT (rw) register accessor: an alias for `Reg<CH11_INTERACT_SPEC>`"]
pub type CH11_INTERACT = crate::Reg<ch11_interact::CH11_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch11_interact;
#[doc = "CH11_EVAL (rw) register accessor: an alias for `Reg<CH11_EVAL_SPEC>`"]
pub type CH11_EVAL = crate::Reg<ch11_eval::CH11_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch11_eval;
#[doc = "CH12_TIMING (rw) register accessor: an alias for `Reg<CH12_TIMING_SPEC>`"]
pub type CH12_TIMING = crate::Reg<ch12_timing::CH12_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch12_timing;
#[doc = "CH12_INTERACT (rw) register accessor: an alias for `Reg<CH12_INTERACT_SPEC>`"]
pub type CH12_INTERACT = crate::Reg<ch12_interact::CH12_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch12_interact;
#[doc = "CH12_EVAL (rw) register accessor: an alias for `Reg<CH12_EVAL_SPEC>`"]
pub type CH12_EVAL = crate::Reg<ch12_eval::CH12_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch12_eval;
#[doc = "CH13_TIMING (rw) register accessor: an alias for `Reg<CH13_TIMING_SPEC>`"]
pub type CH13_TIMING = crate::Reg<ch13_timing::CH13_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch13_timing;
#[doc = "CH13_INTERACT (rw) register accessor: an alias for `Reg<CH13_INTERACT_SPEC>`"]
pub type CH13_INTERACT = crate::Reg<ch13_interact::CH13_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch13_interact;
#[doc = "CH13_EVAL (rw) register accessor: an alias for `Reg<CH13_EVAL_SPEC>`"]
pub type CH13_EVAL = crate::Reg<ch13_eval::CH13_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch13_eval;
#[doc = "CH14_TIMING (rw) register accessor: an alias for `Reg<CH14_TIMING_SPEC>`"]
pub type CH14_TIMING = crate::Reg<ch14_timing::CH14_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch14_timing;
#[doc = "CH14_INTERACT (rw) register accessor: an alias for `Reg<CH14_INTERACT_SPEC>`"]
pub type CH14_INTERACT = crate::Reg<ch14_interact::CH14_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch14_interact;
#[doc = "CH14_EVAL (rw) register accessor: an alias for `Reg<CH14_EVAL_SPEC>`"]
pub type CH14_EVAL = crate::Reg<ch14_eval::CH14_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch14_eval;
#[doc = "CH15_TIMING (rw) register accessor: an alias for `Reg<CH15_TIMING_SPEC>`"]
pub type CH15_TIMING = crate::Reg<ch15_timing::CH15_TIMING_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch15_timing;
#[doc = "CH15_INTERACT (rw) register accessor: an alias for `Reg<CH15_INTERACT_SPEC>`"]
pub type CH15_INTERACT = crate::Reg<ch15_interact::CH15_INTERACT_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch15_interact;
#[doc = "CH15_EVAL (rw) register accessor: an alias for `Reg<CH15_EVAL_SPEC>`"]
pub type CH15_EVAL = crate::Reg<ch15_eval::CH15_EVAL_SPEC>;
#[doc = "Scan Configuration"]
pub mod ch15_eval;
