#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - HFRCO Control Register"]
    pub hfrcoctrl: HFRCOCTRL,
    _reserved2: [u8; 0x04],
    #[doc = "0x18 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: AUXHFRCOCTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - LFRCO Control Register"]
    pub lfrcoctrl: LFRCOCTRL,
    #[doc = "0x24 - HFXO Control Register"]
    pub hfxoctrl: HFXOCTRL,
    #[doc = "0x28 - HFXO Control 1"]
    pub hfxoctrl1: HFXOCTRL1,
    #[doc = "0x2c - HFXO Startup Control"]
    pub hfxostartupctrl: HFXOSTARTUPCTRL,
    #[doc = "0x30 - HFXO Steady State Control"]
    pub hfxosteadystatectrl: HFXOSTEADYSTATECTRL,
    #[doc = "0x34 - HFXO Timeout Control"]
    pub hfxotimeoutctrl: HFXOTIMEOUTCTRL,
    #[doc = "0x38 - LFXO Control Register"]
    pub lfxoctrl: LFXOCTRL,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - DPLL Control Register"]
    pub dpllctrl: DPLLCTRL,
    #[doc = "0x44 - DPLL Control Register"]
    pub dpllctrl1: DPLLCTRL1,
    _reserved12: [u8; 0x08],
    #[doc = "0x50 - Calibration Control Register"]
    pub calctrl: CALCTRL,
    #[doc = "0x54 - Calibration Counter Register"]
    pub calcnt: CALCNT,
    _reserved14: [u8; 0x08],
    #[doc = "0x60 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: OSCENCMD,
    #[doc = "0x64 - Command Register"]
    pub cmd: CMD,
    _reserved16: [u8; 0x08],
    #[doc = "0x70 - Debug Trace Clock Select"]
    pub dbgclksel: DBGCLKSEL,
    #[doc = "0x74 - High Frequency Clock Select Command Register"]
    pub hfclksel: HFCLKSEL,
    _reserved18: [u8; 0x08],
    #[doc = "0x80 - Low Frequency A Clock Select Register"]
    pub lfaclksel: LFACLKSEL,
    #[doc = "0x84 - Low Frequency B Clock Select Register"]
    pub lfbclksel: LFBCLKSEL,
    #[doc = "0x88 - Low Frequency E Clock Select Register"]
    pub lfeclksel: LFECLKSEL,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - Status Register"]
    pub status: STATUS,
    #[doc = "0x94 - HFCLK Status Register"]
    pub hfclkstatus: HFCLKSTATUS,
    _reserved23: [u8; 0x04],
    #[doc = "0x9c - HFXO Trim Status"]
    pub hfxotrimstatus: HFXOTRIMSTATUS,
    #[doc = "0xa0 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0xac - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0xb0 - High Frequency Bus Clock Enable Register 0"]
    pub hfbusclken0: HFBUSCLKEN0,
    _reserved29: [u8; 0x0c],
    #[doc = "0xc0 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: HFPERCLKEN0,
    #[doc = "0xc4 - High Frequency Peripheral Clock Enable Register 1"]
    pub hfperclken1: HFPERCLKEN1,
    _reserved31: [u8; 0x18],
    #[doc = "0xe0 - Low Frequency a Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: LFACLKEN0,
    _reserved32: [u8; 0x04],
    #[doc = "0xe8 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: LFBCLKEN0,
    _reserved33: [u8; 0x04],
    #[doc = "0xf0 - Low Frequency E Clock Enable Register 0 (Async Reg)"]
    pub lfeclken0: LFECLKEN0,
    _reserved34: [u8; 0x0c],
    #[doc = "0x100 - High Frequency Clock Prescaler Register"]
    pub hfpresc: HFPRESC,
    #[doc = "0x104 - High Frequency Bus Clock Prescaler Register"]
    pub hfbuspresc: HFBUSPRESC,
    #[doc = "0x108 - High Frequency Core Clock Prescaler Register"]
    pub hfcorepresc: HFCOREPRESC,
    #[doc = "0x10c - High Frequency Peripheral Clock Prescaler Register"]
    pub hfperpresc: HFPERPRESC,
    _reserved38: [u8; 0x04],
    #[doc = "0x114 - High Frequency Export Clock Prescaler Register"]
    pub hfexppresc: HFEXPPRESC,
    #[doc = "0x118 - High Frequency Peripheral Clock Prescaler B Register"]
    pub hfperprescb: HFPERPRESCB,
    #[doc = "0x11c - High Frequency Peripheral Clock Prescaler C Register"]
    pub hfperprescc: HFPERPRESCC,
    #[doc = "0x120 - Low Frequency a Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: LFAPRESC0,
    _reserved42: [u8; 0x04],
    #[doc = "0x128 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: LFBPRESC0,
    _reserved43: [u8; 0x04],
    #[doc = "0x130 - Low Frequency E Prescaler Register 0 (Async Reg)"]
    pub lfepresc0: LFEPRESC0,
    _reserved44: [u8; 0x0c],
    #[doc = "0x140 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x144 - Freeze Register"]
    pub freeze: FREEZE,
    _reserved46: [u8; 0x08],
    #[doc = "0x150 - PCNT Control Register"]
    pub pcntctrl: PCNTCTRL,
    _reserved47: [u8; 0x08],
    #[doc = "0x15c - ADC Control Register"]
    pub adcctrl: ADCCTRL,
    _reserved48: [u8; 0x10],
    #[doc = "0x170 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x174 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x178 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
    _reserved51: [u8; 0x04],
    #[doc = "0x180 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x184 - HFRCO Spread Spectrum Register"]
    pub hfrcoss: HFRCOSS,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFRCOCTRL (rw) register accessor: an alias for `Reg<HFRCOCTRL_SPEC>`"]
pub type HFRCOCTRL = crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "AUXHFRCOCTRL (rw) register accessor: an alias for `Reg<AUXHFRCOCTRL_SPEC>`"]
pub type AUXHFRCOCTRL = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "LFRCOCTRL (rw) register accessor: an alias for `Reg<LFRCOCTRL_SPEC>`"]
pub type LFRCOCTRL = crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "HFXOCTRL (rw) register accessor: an alias for `Reg<HFXOCTRL_SPEC>`"]
pub type HFXOCTRL = crate::Reg<hfxoctrl::HFXOCTRL_SPEC>;
#[doc = "HFXO Control Register"]
pub mod hfxoctrl;
#[doc = "HFXOCTRL1 (rw) register accessor: an alias for `Reg<HFXOCTRL1_SPEC>`"]
pub type HFXOCTRL1 = crate::Reg<hfxoctrl1::HFXOCTRL1_SPEC>;
#[doc = "HFXO Control 1"]
pub mod hfxoctrl1;
#[doc = "HFXOSTARTUPCTRL (rw) register accessor: an alias for `Reg<HFXOSTARTUPCTRL_SPEC>`"]
pub type HFXOSTARTUPCTRL = crate::Reg<hfxostartupctrl::HFXOSTARTUPCTRL_SPEC>;
#[doc = "HFXO Startup Control"]
pub mod hfxostartupctrl;
#[doc = "HFXOSTEADYSTATECTRL (rw) register accessor: an alias for `Reg<HFXOSTEADYSTATECTRL_SPEC>`"]
pub type HFXOSTEADYSTATECTRL = crate::Reg<hfxosteadystatectrl::HFXOSTEADYSTATECTRL_SPEC>;
#[doc = "HFXO Steady State Control"]
pub mod hfxosteadystatectrl;
#[doc = "HFXOTIMEOUTCTRL (rw) register accessor: an alias for `Reg<HFXOTIMEOUTCTRL_SPEC>`"]
pub type HFXOTIMEOUTCTRL = crate::Reg<hfxotimeoutctrl::HFXOTIMEOUTCTRL_SPEC>;
#[doc = "HFXO Timeout Control"]
pub mod hfxotimeoutctrl;
#[doc = "LFXOCTRL (rw) register accessor: an alias for `Reg<LFXOCTRL_SPEC>`"]
pub type LFXOCTRL = crate::Reg<lfxoctrl::LFXOCTRL_SPEC>;
#[doc = "LFXO Control Register"]
pub mod lfxoctrl;
#[doc = "DPLLCTRL (rw) register accessor: an alias for `Reg<DPLLCTRL_SPEC>`"]
pub type DPLLCTRL = crate::Reg<dpllctrl::DPLLCTRL_SPEC>;
#[doc = "DPLL Control Register"]
pub mod dpllctrl;
#[doc = "DPLLCTRL1 (rw) register accessor: an alias for `Reg<DPLLCTRL1_SPEC>`"]
pub type DPLLCTRL1 = crate::Reg<dpllctrl1::DPLLCTRL1_SPEC>;
#[doc = "DPLL Control Register"]
pub mod dpllctrl1;
#[doc = "CALCTRL (rw) register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT (rw) register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD (w) register accessor: an alias for `Reg<OSCENCMD_SPEC>`"]
pub type OSCENCMD = crate::Reg<oscencmd::OSCENCMD_SPEC>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DBGCLKSEL (rw) register accessor: an alias for `Reg<DBGCLKSEL_SPEC>`"]
pub type DBGCLKSEL = crate::Reg<dbgclksel::DBGCLKSEL_SPEC>;
#[doc = "Debug Trace Clock Select"]
pub mod dbgclksel;
#[doc = "HFCLKSEL (w) register accessor: an alias for `Reg<HFCLKSEL_SPEC>`"]
pub type HFCLKSEL = crate::Reg<hfclksel::HFCLKSEL_SPEC>;
#[doc = "High Frequency Clock Select Command Register"]
pub mod hfclksel;
#[doc = "LFACLKSEL (rw) register accessor: an alias for `Reg<LFACLKSEL_SPEC>`"]
pub type LFACLKSEL = crate::Reg<lfaclksel::LFACLKSEL_SPEC>;
#[doc = "Low Frequency A Clock Select Register"]
pub mod lfaclksel;
#[doc = "LFBCLKSEL (rw) register accessor: an alias for `Reg<LFBCLKSEL_SPEC>`"]
pub type LFBCLKSEL = crate::Reg<lfbclksel::LFBCLKSEL_SPEC>;
#[doc = "Low Frequency B Clock Select Register"]
pub mod lfbclksel;
#[doc = "LFECLKSEL (rw) register accessor: an alias for `Reg<LFECLKSEL_SPEC>`"]
pub type LFECLKSEL = crate::Reg<lfeclksel::LFECLKSEL_SPEC>;
#[doc = "Low Frequency E Clock Select Register"]
pub mod lfeclksel;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "HFCLKSTATUS (r) register accessor: an alias for `Reg<HFCLKSTATUS_SPEC>`"]
pub type HFCLKSTATUS = crate::Reg<hfclkstatus::HFCLKSTATUS_SPEC>;
#[doc = "HFCLK Status Register"]
pub mod hfclkstatus;
#[doc = "HFXOTRIMSTATUS (r) register accessor: an alias for `Reg<HFXOTRIMSTATUS_SPEC>`"]
pub type HFXOTRIMSTATUS = crate::Reg<hfxotrimstatus::HFXOTRIMSTATUS_SPEC>;
#[doc = "HFXO Trim Status"]
pub mod hfxotrimstatus;
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
#[doc = "HFBUSCLKEN0 (rw) register accessor: an alias for `Reg<HFBUSCLKEN0_SPEC>`"]
pub type HFBUSCLKEN0 = crate::Reg<hfbusclken0::HFBUSCLKEN0_SPEC>;
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub mod hfbusclken0;
#[doc = "HFPERCLKEN0 (rw) register accessor: an alias for `Reg<HFPERCLKEN0_SPEC>`"]
pub type HFPERCLKEN0 = crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "HFPERCLKEN1 (rw) register accessor: an alias for `Reg<HFPERCLKEN1_SPEC>`"]
pub type HFPERCLKEN1 = crate::Reg<hfperclken1::HFPERCLKEN1_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 1"]
pub mod hfperclken1;
#[doc = "LFACLKEN0 (rw) register accessor: an alias for `Reg<LFACLKEN0_SPEC>`"]
pub type LFACLKEN0 = crate::Reg<lfaclken0::LFACLKEN0_SPEC>;
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 (rw) register accessor: an alias for `Reg<LFBCLKEN0_SPEC>`"]
pub type LFBCLKEN0 = crate::Reg<lfbclken0::LFBCLKEN0_SPEC>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFECLKEN0 (rw) register accessor: an alias for `Reg<LFECLKEN0_SPEC>`"]
pub type LFECLKEN0 = crate::Reg<lfeclken0::LFECLKEN0_SPEC>;
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub mod lfeclken0;
#[doc = "HFPRESC (rw) register accessor: an alias for `Reg<HFPRESC_SPEC>`"]
pub type HFPRESC = crate::Reg<hfpresc::HFPRESC_SPEC>;
#[doc = "High Frequency Clock Prescaler Register"]
pub mod hfpresc;
#[doc = "HFBUSPRESC (rw) register accessor: an alias for `Reg<HFBUSPRESC_SPEC>`"]
pub type HFBUSPRESC = crate::Reg<hfbuspresc::HFBUSPRESC_SPEC>;
#[doc = "High Frequency Bus Clock Prescaler Register"]
pub mod hfbuspresc;
#[doc = "HFCOREPRESC (rw) register accessor: an alias for `Reg<HFCOREPRESC_SPEC>`"]
pub type HFCOREPRESC = crate::Reg<hfcorepresc::HFCOREPRESC_SPEC>;
#[doc = "High Frequency Core Clock Prescaler Register"]
pub mod hfcorepresc;
#[doc = "HFPERPRESC (rw) register accessor: an alias for `Reg<HFPERPRESC_SPEC>`"]
pub type HFPERPRESC = crate::Reg<hfperpresc::HFPERPRESC_SPEC>;
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub mod hfperpresc;
#[doc = "HFEXPPRESC (rw) register accessor: an alias for `Reg<HFEXPPRESC_SPEC>`"]
pub type HFEXPPRESC = crate::Reg<hfexppresc::HFEXPPRESC_SPEC>;
#[doc = "High Frequency Export Clock Prescaler Register"]
pub mod hfexppresc;
#[doc = "HFPERPRESCB (rw) register accessor: an alias for `Reg<HFPERPRESCB_SPEC>`"]
pub type HFPERPRESCB = crate::Reg<hfperprescb::HFPERPRESCB_SPEC>;
#[doc = "High Frequency Peripheral Clock Prescaler B Register"]
pub mod hfperprescb;
#[doc = "HFPERPRESCC (rw) register accessor: an alias for `Reg<HFPERPRESCC_SPEC>`"]
pub type HFPERPRESCC = crate::Reg<hfperprescc::HFPERPRESCC_SPEC>;
#[doc = "High Frequency Peripheral Clock Prescaler C Register"]
pub mod hfperprescc;
#[doc = "LFAPRESC0 (rw) register accessor: an alias for `Reg<LFAPRESC0_SPEC>`"]
pub type LFAPRESC0 = crate::Reg<lfapresc0::LFAPRESC0_SPEC>;
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 (rw) register accessor: an alias for `Reg<LFBPRESC0_SPEC>`"]
pub type LFBPRESC0 = crate::Reg<lfbpresc0::LFBPRESC0_SPEC>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "LFEPRESC0 (rw) register accessor: an alias for `Reg<LFEPRESC0_SPEC>`"]
pub type LFEPRESC0 = crate::Reg<lfepresc0::LFEPRESC0_SPEC>;
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub mod lfepresc0;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE (rw) register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "PCNTCTRL (rw) register accessor: an alias for `Reg<PCNTCTRL_SPEC>`"]
pub type PCNTCTRL = crate::Reg<pcntctrl::PCNTCTRL_SPEC>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ADCCTRL (rw) register accessor: an alias for `Reg<ADCCTRL_SPEC>`"]
pub type ADCCTRL = crate::Reg<adcctrl::ADCCTRL_SPEC>;
#[doc = "ADC Control Register"]
pub mod adcctrl;
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
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "HFRCOSS (rw) register accessor: an alias for `Reg<HFRCOSS_SPEC>`"]
pub type HFRCOSS = crate::Reg<hfrcoss::HFRCOSS_SPEC>;
#[doc = "HFRCO Spread Spectrum Register"]
pub mod hfrcoss;
