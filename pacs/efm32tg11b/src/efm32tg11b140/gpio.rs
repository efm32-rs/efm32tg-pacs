#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub pa_ctrl: PA_CTRL,
    #[doc = "0x04 - Port Pin Mode Low Register"]
    pub pa_model: PA_MODEL,
    #[doc = "0x08 - Port Pin Mode High Register"]
    pub pa_modeh: PA_MODEH,
    #[doc = "0x0c - Port Data Out Register"]
    pub pa_dout: PA_DOUT,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Port Data Out Toggle Register"]
    pub pa_douttgl: PA_DOUTTGL,
    #[doc = "0x1c - Port Data in Register"]
    pub pa_din: PA_DIN,
    #[doc = "0x20 - Port Unlocked Pins Register"]
    pub pa_pinlockn: PA_PINLOCKN,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - Over Voltage Disable for All Modes"]
    pub pa_ovtdis: PA_OVTDIS,
    _reserved8: [u8; 0x04],
    #[doc = "0x30 - Port Control Register"]
    pub pb_ctrl: PB_CTRL,
    #[doc = "0x34 - Port Pin Mode Low Register"]
    pub pb_model: PB_MODEL,
    #[doc = "0x38 - Port Pin Mode High Register"]
    pub pb_modeh: PB_MODEH,
    #[doc = "0x3c - Port Data Out Register"]
    pub pb_dout: PB_DOUT,
    _reserved12: [u8; 0x08],
    #[doc = "0x48 - Port Data Out Toggle Register"]
    pub pb_douttgl: PB_DOUTTGL,
    #[doc = "0x4c - Port Data in Register"]
    pub pb_din: PB_DIN,
    #[doc = "0x50 - Port Unlocked Pins Register"]
    pub pb_pinlockn: PB_PINLOCKN,
    _reserved15: [u8; 0x04],
    #[doc = "0x58 - Over Voltage Disable for All Modes"]
    pub pb_ovtdis: PB_OVTDIS,
    _reserved16: [u8; 0x04],
    #[doc = "0x60 - Port Control Register"]
    pub pc_ctrl: PC_CTRL,
    #[doc = "0x64 - Port Pin Mode Low Register"]
    pub pc_model: PC_MODEL,
    #[doc = "0x68 - Port Pin Mode High Register"]
    pub pc_modeh: PC_MODEH,
    #[doc = "0x6c - Port Data Out Register"]
    pub pc_dout: PC_DOUT,
    _reserved20: [u8; 0x08],
    #[doc = "0x78 - Port Data Out Toggle Register"]
    pub pc_douttgl: PC_DOUTTGL,
    #[doc = "0x7c - Port Data in Register"]
    pub pc_din: PC_DIN,
    #[doc = "0x80 - Port Unlocked Pins Register"]
    pub pc_pinlockn: PC_PINLOCKN,
    _reserved23: [u8; 0x04],
    #[doc = "0x88 - Over Voltage Disable for All Modes"]
    pub pc_ovtdis: PC_OVTDIS,
    _reserved24: [u8; 0x04],
    #[doc = "0x90 - Port Control Register"]
    pub pd_ctrl: PD_CTRL,
    #[doc = "0x94 - Port Pin Mode Low Register"]
    pub pd_model: PD_MODEL,
    #[doc = "0x98 - Port Pin Mode High Register"]
    pub pd_modeh: PD_MODEH,
    #[doc = "0x9c - Port Data Out Register"]
    pub pd_dout: PD_DOUT,
    _reserved28: [u8; 0x08],
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    pub pd_douttgl: PD_DOUTTGL,
    #[doc = "0xac - Port Data in Register"]
    pub pd_din: PD_DIN,
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    pub pd_pinlockn: PD_PINLOCKN,
    _reserved31: [u8; 0x04],
    #[doc = "0xb8 - Over Voltage Disable for All Modes"]
    pub pd_ovtdis: PD_OVTDIS,
    _reserved32: [u8; 0x04],
    #[doc = "0xc0 - Port Control Register"]
    pub pe_ctrl: PE_CTRL,
    #[doc = "0xc4 - Port Pin Mode Low Register"]
    pub pe_model: PE_MODEL,
    #[doc = "0xc8 - Port Pin Mode High Register"]
    pub pe_modeh: PE_MODEH,
    #[doc = "0xcc - Port Data Out Register"]
    pub pe_dout: PE_DOUT,
    _reserved36: [u8; 0x08],
    #[doc = "0xd8 - Port Data Out Toggle Register"]
    pub pe_douttgl: PE_DOUTTGL,
    #[doc = "0xdc - Port Data in Register"]
    pub pe_din: PE_DIN,
    #[doc = "0xe0 - Port Unlocked Pins Register"]
    pub pe_pinlockn: PE_PINLOCKN,
    _reserved39: [u8; 0x04],
    #[doc = "0xe8 - Over Voltage Disable for All Modes"]
    pub pe_ovtdis: PE_OVTDIS,
    _reserved40: [u8; 0x04],
    #[doc = "0xf0 - Port Control Register"]
    pub pf_ctrl: PF_CTRL,
    #[doc = "0xf4 - Port Pin Mode Low Register"]
    pub pf_model: PF_MODEL,
    #[doc = "0xf8 - Port Pin Mode High Register"]
    pub pf_modeh: PF_MODEH,
    #[doc = "0xfc - Port Data Out Register"]
    pub pf_dout: PF_DOUT,
    _reserved44: [u8; 0x08],
    #[doc = "0x108 - Port Data Out Toggle Register"]
    pub pf_douttgl: PF_DOUTTGL,
    #[doc = "0x10c - Port Data in Register"]
    pub pf_din: PF_DIN,
    #[doc = "0x110 - Port Unlocked Pins Register"]
    pub pf_pinlockn: PF_PINLOCKN,
    _reserved47: [u8; 0x04],
    #[doc = "0x118 - Over Voltage Disable for All Modes"]
    pub pf_ovtdis: PF_OVTDIS,
    _reserved48: [u8; 0x04],
    #[doc = "0x120 - Port Control Register"]
    pub pg_ctrl: PG_CTRL,
    #[doc = "0x124 - Port Pin Mode Low Register"]
    pub pg_model: PG_MODEL,
    #[doc = "0x128 - Port Pin Mode High Register"]
    pub pg_modeh: PG_MODEH,
    #[doc = "0x12c - Port Data Out Register"]
    pub pg_dout: PG_DOUT,
    _reserved52: [u8; 0x08],
    #[doc = "0x138 - Port Data Out Toggle Register"]
    pub pg_douttgl: PG_DOUTTGL,
    #[doc = "0x13c - Port Data in Register"]
    pub pg_din: PG_DIN,
    #[doc = "0x140 - Port Unlocked Pins Register"]
    pub pg_pinlockn: PG_PINLOCKN,
    _reserved55: [u8; 0x04],
    #[doc = "0x148 - Over Voltage Disable for All Modes"]
    pub pg_ovtdis: PG_OVTDIS,
    _reserved56: [u8; 0x04],
    #[doc = "0x150 - Port Control Register"]
    pub ph_ctrl: PH_CTRL,
    #[doc = "0x154 - Port Pin Mode Low Register"]
    pub ph_model: PH_MODEL,
    #[doc = "0x158 - Port Pin Mode High Register"]
    pub ph_modeh: PH_MODEH,
    #[doc = "0x15c - Port Data Out Register"]
    pub ph_dout: PH_DOUT,
    _reserved60: [u8; 0x08],
    #[doc = "0x168 - Port Data Out Toggle Register"]
    pub ph_douttgl: PH_DOUTTGL,
    #[doc = "0x16c - Port Data in Register"]
    pub ph_din: PH_DIN,
    #[doc = "0x170 - Port Unlocked Pins Register"]
    pub ph_pinlockn: PH_PINLOCKN,
    _reserved63: [u8; 0x04],
    #[doc = "0x178 - Over Voltage Disable for All Modes"]
    pub ph_ovtdis: PH_OVTDIS,
    _reserved64: [u8; 0x04],
    #[doc = "0x180 - Port Control Register"]
    pub pi_ctrl: PI_CTRL,
    #[doc = "0x184 - Port Pin Mode Low Register"]
    pub pi_model: PI_MODEL,
    #[doc = "0x188 - Port Pin Mode High Register"]
    pub pi_modeh: PI_MODEH,
    #[doc = "0x18c - Port Data Out Register"]
    pub pi_dout: PI_DOUT,
    _reserved68: [u8; 0x08],
    #[doc = "0x198 - Port Data Out Toggle Register"]
    pub pi_douttgl: PI_DOUTTGL,
    #[doc = "0x19c - Port Data in Register"]
    pub pi_din: PI_DIN,
    #[doc = "0x1a0 - Port Unlocked Pins Register"]
    pub pi_pinlockn: PI_PINLOCKN,
    _reserved71: [u8; 0x04],
    #[doc = "0x1a8 - Over Voltage Disable for All Modes"]
    pub pi_ovtdis: PI_OVTDIS,
    _reserved72: [u8; 0x04],
    #[doc = "0x1b0 - Port Control Register"]
    pub pj_ctrl: PJ_CTRL,
    #[doc = "0x1b4 - Port Pin Mode Low Register"]
    pub pj_model: PJ_MODEL,
    #[doc = "0x1b8 - Port Pin Mode High Register"]
    pub pj_modeh: PJ_MODEH,
    #[doc = "0x1bc - Port Data Out Register"]
    pub pj_dout: PJ_DOUT,
    _reserved76: [u8; 0x08],
    #[doc = "0x1c8 - Port Data Out Toggle Register"]
    pub pj_douttgl: PJ_DOUTTGL,
    #[doc = "0x1cc - Port Data in Register"]
    pub pj_din: PJ_DIN,
    #[doc = "0x1d0 - Port Unlocked Pins Register"]
    pub pj_pinlockn: PJ_PINLOCKN,
    _reserved79: [u8; 0x04],
    #[doc = "0x1d8 - Over Voltage Disable for All Modes"]
    pub pj_ovtdis: PJ_OVTDIS,
    _reserved80: [u8; 0x04],
    #[doc = "0x1e0 - Port Control Register"]
    pub pk_ctrl: PK_CTRL,
    #[doc = "0x1e4 - Port Pin Mode Low Register"]
    pub pk_model: PK_MODEL,
    #[doc = "0x1e8 - Port Pin Mode High Register"]
    pub pk_modeh: PK_MODEH,
    #[doc = "0x1ec - Port Data Out Register"]
    pub pk_dout: PK_DOUT,
    _reserved84: [u8; 0x08],
    #[doc = "0x1f8 - Port Data Out Toggle Register"]
    pub pk_douttgl: PK_DOUTTGL,
    #[doc = "0x1fc - Port Data in Register"]
    pub pk_din: PK_DIN,
    #[doc = "0x200 - Port Unlocked Pins Register"]
    pub pk_pinlockn: PK_PINLOCKN,
    _reserved87: [u8; 0x04],
    #[doc = "0x208 - Over Voltage Disable for All Modes"]
    pub pk_ovtdis: PK_OVTDIS,
    _reserved88: [u8; 0x04],
    #[doc = "0x210 - Port Control Register"]
    pub pl_ctrl: PL_CTRL,
    #[doc = "0x214 - Port Pin Mode Low Register"]
    pub pl_model: PL_MODEL,
    #[doc = "0x218 - Port Pin Mode High Register"]
    pub pl_modeh: PL_MODEH,
    #[doc = "0x21c - Port Data Out Register"]
    pub pl_dout: PL_DOUT,
    _reserved92: [u8; 0x08],
    #[doc = "0x228 - Port Data Out Toggle Register"]
    pub pl_douttgl: PL_DOUTTGL,
    #[doc = "0x22c - Port Data in Register"]
    pub pl_din: PL_DIN,
    #[doc = "0x230 - Port Unlocked Pins Register"]
    pub pl_pinlockn: PL_PINLOCKN,
    _reserved95: [u8; 0x04],
    #[doc = "0x238 - Over Voltage Disable for All Modes"]
    pub pl_ovtdis: PL_OVTDIS,
    _reserved96: [u8; 0x01c4],
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    pub extipinsell: EXTIPINSELL,
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    pub extipinselh: EXTIPINSELH,
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    pub extirise: EXTIRISE,
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    pub extifall: EXTIFALL,
    #[doc = "0x418 - External Interrupt Level Register"]
    pub extilevel: EXTILEVEL,
    #[doc = "0x41c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x420 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x428 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    pub em4wuen: EM4WUEN,
    _reserved108: [u8; 0x10],
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    _reserved109: [u8; 0x0c],
    #[doc = "0x450 - Input Sense Register"]
    pub insense: INSENSE,
    #[doc = "0x454 - Configuration Lock Register"]
    pub lock: LOCK,
}
#[doc = "PA_CTRL (rw) register accessor: an alias for `Reg<PA_CTRL_SPEC>`"]
pub type PA_CTRL = crate::Reg<pa_ctrl::PA_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL (rw) register accessor: an alias for `Reg<PA_MODEL_SPEC>`"]
pub type PA_MODEL = crate::Reg<pa_model::PA_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH (rw) register accessor: an alias for `Reg<PA_MODEH_SPEC>`"]
pub type PA_MODEH = crate::Reg<pa_modeh::PA_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT (rw) register accessor: an alias for `Reg<PA_DOUT_SPEC>`"]
pub type PA_DOUT = crate::Reg<pa_dout::PA_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTTGL (w) register accessor: an alias for `Reg<PA_DOUTTGL_SPEC>`"]
pub type PA_DOUTTGL = crate::Reg<pa_douttgl::PA_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN (r) register accessor: an alias for `Reg<PA_DIN_SPEC>`"]
pub type PA_DIN = crate::Reg<pa_din::PA_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN (rw) register accessor: an alias for `Reg<PA_PINLOCKN_SPEC>`"]
pub type PA_PINLOCKN = crate::Reg<pa_pinlockn::PA_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PA_OVTDIS (rw) register accessor: an alias for `Reg<PA_OVTDIS_SPEC>`"]
pub type PA_OVTDIS = crate::Reg<pa_ovtdis::PA_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pa_ovtdis;
#[doc = "PB_CTRL (rw) register accessor: an alias for `Reg<PB_CTRL_SPEC>`"]
pub type PB_CTRL = crate::Reg<pb_ctrl::PB_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL (rw) register accessor: an alias for `Reg<PB_MODEL_SPEC>`"]
pub type PB_MODEL = crate::Reg<pb_model::PB_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH (rw) register accessor: an alias for `Reg<PB_MODEH_SPEC>`"]
pub type PB_MODEH = crate::Reg<pb_modeh::PB_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT (rw) register accessor: an alias for `Reg<PB_DOUT_SPEC>`"]
pub type PB_DOUT = crate::Reg<pb_dout::PB_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTTGL (w) register accessor: an alias for `Reg<PB_DOUTTGL_SPEC>`"]
pub type PB_DOUTTGL = crate::Reg<pb_douttgl::PB_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN (r) register accessor: an alias for `Reg<PB_DIN_SPEC>`"]
pub type PB_DIN = crate::Reg<pb_din::PB_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN (rw) register accessor: an alias for `Reg<PB_PINLOCKN_SPEC>`"]
pub type PB_PINLOCKN = crate::Reg<pb_pinlockn::PB_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PB_OVTDIS (rw) register accessor: an alias for `Reg<PB_OVTDIS_SPEC>`"]
pub type PB_OVTDIS = crate::Reg<pb_ovtdis::PB_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pb_ovtdis;
#[doc = "PC_CTRL (rw) register accessor: an alias for `Reg<PC_CTRL_SPEC>`"]
pub type PC_CTRL = crate::Reg<pc_ctrl::PC_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL (rw) register accessor: an alias for `Reg<PC_MODEL_SPEC>`"]
pub type PC_MODEL = crate::Reg<pc_model::PC_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH (rw) register accessor: an alias for `Reg<PC_MODEH_SPEC>`"]
pub type PC_MODEH = crate::Reg<pc_modeh::PC_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT (rw) register accessor: an alias for `Reg<PC_DOUT_SPEC>`"]
pub type PC_DOUT = crate::Reg<pc_dout::PC_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTTGL (w) register accessor: an alias for `Reg<PC_DOUTTGL_SPEC>`"]
pub type PC_DOUTTGL = crate::Reg<pc_douttgl::PC_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN (r) register accessor: an alias for `Reg<PC_DIN_SPEC>`"]
pub type PC_DIN = crate::Reg<pc_din::PC_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN (rw) register accessor: an alias for `Reg<PC_PINLOCKN_SPEC>`"]
pub type PC_PINLOCKN = crate::Reg<pc_pinlockn::PC_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PC_OVTDIS (rw) register accessor: an alias for `Reg<PC_OVTDIS_SPEC>`"]
pub type PC_OVTDIS = crate::Reg<pc_ovtdis::PC_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pc_ovtdis;
#[doc = "PD_CTRL (rw) register accessor: an alias for `Reg<PD_CTRL_SPEC>`"]
pub type PD_CTRL = crate::Reg<pd_ctrl::PD_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL (rw) register accessor: an alias for `Reg<PD_MODEL_SPEC>`"]
pub type PD_MODEL = crate::Reg<pd_model::PD_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH (rw) register accessor: an alias for `Reg<PD_MODEH_SPEC>`"]
pub type PD_MODEH = crate::Reg<pd_modeh::PD_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT (rw) register accessor: an alias for `Reg<PD_DOUT_SPEC>`"]
pub type PD_DOUT = crate::Reg<pd_dout::PD_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTTGL (w) register accessor: an alias for `Reg<PD_DOUTTGL_SPEC>`"]
pub type PD_DOUTTGL = crate::Reg<pd_douttgl::PD_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN (r) register accessor: an alias for `Reg<PD_DIN_SPEC>`"]
pub type PD_DIN = crate::Reg<pd_din::PD_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN (rw) register accessor: an alias for `Reg<PD_PINLOCKN_SPEC>`"]
pub type PD_PINLOCKN = crate::Reg<pd_pinlockn::PD_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PD_OVTDIS (rw) register accessor: an alias for `Reg<PD_OVTDIS_SPEC>`"]
pub type PD_OVTDIS = crate::Reg<pd_ovtdis::PD_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pd_ovtdis;
#[doc = "PE_CTRL (rw) register accessor: an alias for `Reg<PE_CTRL_SPEC>`"]
pub type PE_CTRL = crate::Reg<pe_ctrl::PE_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL (rw) register accessor: an alias for `Reg<PE_MODEL_SPEC>`"]
pub type PE_MODEL = crate::Reg<pe_model::PE_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH (rw) register accessor: an alias for `Reg<PE_MODEH_SPEC>`"]
pub type PE_MODEH = crate::Reg<pe_modeh::PE_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT (rw) register accessor: an alias for `Reg<PE_DOUT_SPEC>`"]
pub type PE_DOUT = crate::Reg<pe_dout::PE_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTTGL (w) register accessor: an alias for `Reg<PE_DOUTTGL_SPEC>`"]
pub type PE_DOUTTGL = crate::Reg<pe_douttgl::PE_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN (r) register accessor: an alias for `Reg<PE_DIN_SPEC>`"]
pub type PE_DIN = crate::Reg<pe_din::PE_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN (rw) register accessor: an alias for `Reg<PE_PINLOCKN_SPEC>`"]
pub type PE_PINLOCKN = crate::Reg<pe_pinlockn::PE_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PE_OVTDIS (rw) register accessor: an alias for `Reg<PE_OVTDIS_SPEC>`"]
pub type PE_OVTDIS = crate::Reg<pe_ovtdis::PE_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pe_ovtdis;
#[doc = "PF_CTRL (rw) register accessor: an alias for `Reg<PF_CTRL_SPEC>`"]
pub type PF_CTRL = crate::Reg<pf_ctrl::PF_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL (rw) register accessor: an alias for `Reg<PF_MODEL_SPEC>`"]
pub type PF_MODEL = crate::Reg<pf_model::PF_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH (rw) register accessor: an alias for `Reg<PF_MODEH_SPEC>`"]
pub type PF_MODEH = crate::Reg<pf_modeh::PF_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT (rw) register accessor: an alias for `Reg<PF_DOUT_SPEC>`"]
pub type PF_DOUT = crate::Reg<pf_dout::PF_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTTGL (w) register accessor: an alias for `Reg<PF_DOUTTGL_SPEC>`"]
pub type PF_DOUTTGL = crate::Reg<pf_douttgl::PF_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN (r) register accessor: an alias for `Reg<PF_DIN_SPEC>`"]
pub type PF_DIN = crate::Reg<pf_din::PF_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN (rw) register accessor: an alias for `Reg<PF_PINLOCKN_SPEC>`"]
pub type PF_PINLOCKN = crate::Reg<pf_pinlockn::PF_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "PF_OVTDIS (rw) register accessor: an alias for `Reg<PF_OVTDIS_SPEC>`"]
pub type PF_OVTDIS = crate::Reg<pf_ovtdis::PF_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pf_ovtdis;
#[doc = "PG_CTRL (rw) register accessor: an alias for `Reg<PG_CTRL_SPEC>`"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pg_ctrl;
#[doc = "PG_MODEL (rw) register accessor: an alias for `Reg<PG_MODEL_SPEC>`"]
pub type PG_MODEL = crate::Reg<pg_model::PG_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pg_model;
#[doc = "PG_MODEH (rw) register accessor: an alias for `Reg<PG_MODEH_SPEC>`"]
pub type PG_MODEH = crate::Reg<pg_modeh::PG_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pg_modeh;
#[doc = "PG_DOUT (rw) register accessor: an alias for `Reg<PG_DOUT_SPEC>`"]
pub type PG_DOUT = crate::Reg<pg_dout::PG_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pg_dout;
#[doc = "PG_DOUTTGL (w) register accessor: an alias for `Reg<PG_DOUTTGL_SPEC>`"]
pub type PG_DOUTTGL = crate::Reg<pg_douttgl::PG_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pg_douttgl;
#[doc = "PG_DIN (r) register accessor: an alias for `Reg<PG_DIN_SPEC>`"]
pub type PG_DIN = crate::Reg<pg_din::PG_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pg_din;
#[doc = "PG_PINLOCKN (rw) register accessor: an alias for `Reg<PG_PINLOCKN_SPEC>`"]
pub type PG_PINLOCKN = crate::Reg<pg_pinlockn::PG_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pg_pinlockn;
#[doc = "PG_OVTDIS (rw) register accessor: an alias for `Reg<PG_OVTDIS_SPEC>`"]
pub type PG_OVTDIS = crate::Reg<pg_ovtdis::PG_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pg_ovtdis;
#[doc = "PH_CTRL (rw) register accessor: an alias for `Reg<PH_CTRL_SPEC>`"]
pub type PH_CTRL = crate::Reg<ph_ctrl::PH_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod ph_ctrl;
#[doc = "PH_MODEL (rw) register accessor: an alias for `Reg<PH_MODEL_SPEC>`"]
pub type PH_MODEL = crate::Reg<ph_model::PH_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod ph_model;
#[doc = "PH_MODEH (rw) register accessor: an alias for `Reg<PH_MODEH_SPEC>`"]
pub type PH_MODEH = crate::Reg<ph_modeh::PH_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod ph_modeh;
#[doc = "PH_DOUT (rw) register accessor: an alias for `Reg<PH_DOUT_SPEC>`"]
pub type PH_DOUT = crate::Reg<ph_dout::PH_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod ph_dout;
#[doc = "PH_DOUTTGL (w) register accessor: an alias for `Reg<PH_DOUTTGL_SPEC>`"]
pub type PH_DOUTTGL = crate::Reg<ph_douttgl::PH_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod ph_douttgl;
#[doc = "PH_DIN (r) register accessor: an alias for `Reg<PH_DIN_SPEC>`"]
pub type PH_DIN = crate::Reg<ph_din::PH_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod ph_din;
#[doc = "PH_PINLOCKN (rw) register accessor: an alias for `Reg<PH_PINLOCKN_SPEC>`"]
pub type PH_PINLOCKN = crate::Reg<ph_pinlockn::PH_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod ph_pinlockn;
#[doc = "PH_OVTDIS (rw) register accessor: an alias for `Reg<PH_OVTDIS_SPEC>`"]
pub type PH_OVTDIS = crate::Reg<ph_ovtdis::PH_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod ph_ovtdis;
#[doc = "PI_CTRL (rw) register accessor: an alias for `Reg<PI_CTRL_SPEC>`"]
pub type PI_CTRL = crate::Reg<pi_ctrl::PI_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pi_ctrl;
#[doc = "PI_MODEL (rw) register accessor: an alias for `Reg<PI_MODEL_SPEC>`"]
pub type PI_MODEL = crate::Reg<pi_model::PI_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pi_model;
#[doc = "PI_MODEH (rw) register accessor: an alias for `Reg<PI_MODEH_SPEC>`"]
pub type PI_MODEH = crate::Reg<pi_modeh::PI_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pi_modeh;
#[doc = "PI_DOUT (rw) register accessor: an alias for `Reg<PI_DOUT_SPEC>`"]
pub type PI_DOUT = crate::Reg<pi_dout::PI_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pi_dout;
#[doc = "PI_DOUTTGL (w) register accessor: an alias for `Reg<PI_DOUTTGL_SPEC>`"]
pub type PI_DOUTTGL = crate::Reg<pi_douttgl::PI_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pi_douttgl;
#[doc = "PI_DIN (r) register accessor: an alias for `Reg<PI_DIN_SPEC>`"]
pub type PI_DIN = crate::Reg<pi_din::PI_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pi_din;
#[doc = "PI_PINLOCKN (rw) register accessor: an alias for `Reg<PI_PINLOCKN_SPEC>`"]
pub type PI_PINLOCKN = crate::Reg<pi_pinlockn::PI_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pi_pinlockn;
#[doc = "PI_OVTDIS (rw) register accessor: an alias for `Reg<PI_OVTDIS_SPEC>`"]
pub type PI_OVTDIS = crate::Reg<pi_ovtdis::PI_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pi_ovtdis;
#[doc = "PJ_CTRL (rw) register accessor: an alias for `Reg<PJ_CTRL_SPEC>`"]
pub type PJ_CTRL = crate::Reg<pj_ctrl::PJ_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pj_ctrl;
#[doc = "PJ_MODEL (rw) register accessor: an alias for `Reg<PJ_MODEL_SPEC>`"]
pub type PJ_MODEL = crate::Reg<pj_model::PJ_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pj_model;
#[doc = "PJ_MODEH (rw) register accessor: an alias for `Reg<PJ_MODEH_SPEC>`"]
pub type PJ_MODEH = crate::Reg<pj_modeh::PJ_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pj_modeh;
#[doc = "PJ_DOUT (rw) register accessor: an alias for `Reg<PJ_DOUT_SPEC>`"]
pub type PJ_DOUT = crate::Reg<pj_dout::PJ_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pj_dout;
#[doc = "PJ_DOUTTGL (w) register accessor: an alias for `Reg<PJ_DOUTTGL_SPEC>`"]
pub type PJ_DOUTTGL = crate::Reg<pj_douttgl::PJ_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pj_douttgl;
#[doc = "PJ_DIN (r) register accessor: an alias for `Reg<PJ_DIN_SPEC>`"]
pub type PJ_DIN = crate::Reg<pj_din::PJ_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pj_din;
#[doc = "PJ_PINLOCKN (rw) register accessor: an alias for `Reg<PJ_PINLOCKN_SPEC>`"]
pub type PJ_PINLOCKN = crate::Reg<pj_pinlockn::PJ_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pj_pinlockn;
#[doc = "PJ_OVTDIS (rw) register accessor: an alias for `Reg<PJ_OVTDIS_SPEC>`"]
pub type PJ_OVTDIS = crate::Reg<pj_ovtdis::PJ_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pj_ovtdis;
#[doc = "PK_CTRL (rw) register accessor: an alias for `Reg<PK_CTRL_SPEC>`"]
pub type PK_CTRL = crate::Reg<pk_ctrl::PK_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pk_ctrl;
#[doc = "PK_MODEL (rw) register accessor: an alias for `Reg<PK_MODEL_SPEC>`"]
pub type PK_MODEL = crate::Reg<pk_model::PK_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pk_model;
#[doc = "PK_MODEH (rw) register accessor: an alias for `Reg<PK_MODEH_SPEC>`"]
pub type PK_MODEH = crate::Reg<pk_modeh::PK_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pk_modeh;
#[doc = "PK_DOUT (rw) register accessor: an alias for `Reg<PK_DOUT_SPEC>`"]
pub type PK_DOUT = crate::Reg<pk_dout::PK_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pk_dout;
#[doc = "PK_DOUTTGL (w) register accessor: an alias for `Reg<PK_DOUTTGL_SPEC>`"]
pub type PK_DOUTTGL = crate::Reg<pk_douttgl::PK_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pk_douttgl;
#[doc = "PK_DIN (r) register accessor: an alias for `Reg<PK_DIN_SPEC>`"]
pub type PK_DIN = crate::Reg<pk_din::PK_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pk_din;
#[doc = "PK_PINLOCKN (rw) register accessor: an alias for `Reg<PK_PINLOCKN_SPEC>`"]
pub type PK_PINLOCKN = crate::Reg<pk_pinlockn::PK_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pk_pinlockn;
#[doc = "PK_OVTDIS (rw) register accessor: an alias for `Reg<PK_OVTDIS_SPEC>`"]
pub type PK_OVTDIS = crate::Reg<pk_ovtdis::PK_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pk_ovtdis;
#[doc = "PL_CTRL (rw) register accessor: an alias for `Reg<PL_CTRL_SPEC>`"]
pub type PL_CTRL = crate::Reg<pl_ctrl::PL_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pl_ctrl;
#[doc = "PL_MODEL (rw) register accessor: an alias for `Reg<PL_MODEL_SPEC>`"]
pub type PL_MODEL = crate::Reg<pl_model::PL_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pl_model;
#[doc = "PL_MODEH (rw) register accessor: an alias for `Reg<PL_MODEH_SPEC>`"]
pub type PL_MODEH = crate::Reg<pl_modeh::PL_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pl_modeh;
#[doc = "PL_DOUT (rw) register accessor: an alias for `Reg<PL_DOUT_SPEC>`"]
pub type PL_DOUT = crate::Reg<pl_dout::PL_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pl_dout;
#[doc = "PL_DOUTTGL (w) register accessor: an alias for `Reg<PL_DOUTTGL_SPEC>`"]
pub type PL_DOUTTGL = crate::Reg<pl_douttgl::PL_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pl_douttgl;
#[doc = "PL_DIN (r) register accessor: an alias for `Reg<PL_DIN_SPEC>`"]
pub type PL_DIN = crate::Reg<pl_din::PL_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pl_din;
#[doc = "PL_PINLOCKN (rw) register accessor: an alias for `Reg<PL_PINLOCKN_SPEC>`"]
pub type PL_PINLOCKN = crate::Reg<pl_pinlockn::PL_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pl_pinlockn;
#[doc = "PL_OVTDIS (rw) register accessor: an alias for `Reg<PL_OVTDIS_SPEC>`"]
pub type PL_OVTDIS = crate::Reg<pl_ovtdis::PL_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pl_ovtdis;
#[doc = "EXTIPSELL (rw) register accessor: an alias for `Reg<EXTIPSELL_SPEC>`"]
pub type EXTIPSELL = crate::Reg<extipsell::EXTIPSELL_SPEC>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: an alias for `Reg<EXTIPSELH_SPEC>`"]
pub type EXTIPSELH = crate::Reg<extipselh::EXTIPSELH_SPEC>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: an alias for `Reg<EXTIPINSELL_SPEC>`"]
pub type EXTIPINSELL = crate::Reg<extipinsell::EXTIPINSELL_SPEC>;
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: an alias for `Reg<EXTIPINSELH_SPEC>`"]
pub type EXTIPINSELH = crate::Reg<extipinselh::EXTIPINSELH_SPEC>;
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: an alias for `Reg<EXTIRISE_SPEC>`"]
pub type EXTIRISE = crate::Reg<extirise::EXTIRISE_SPEC>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: an alias for `Reg<EXTIFALL_SPEC>`"]
pub type EXTIFALL = crate::Reg<extifall::EXTIFALL_SPEC>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "EXTILEVEL (rw) register accessor: an alias for `Reg<EXTILEVEL_SPEC>`"]
pub type EXTILEVEL = crate::Reg<extilevel::EXTILEVEL_SPEC>;
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
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
#[doc = "EM4WUEN (rw) register accessor: an alias for `Reg<EM4WUEN_SPEC>`"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
#[doc = "ROUTEPEN (rw) register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "INSENSE (rw) register accessor: an alias for `Reg<INSENSE_SPEC>`"]
pub type INSENSE = crate::Reg<insense::INSENSE_SPEC>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
