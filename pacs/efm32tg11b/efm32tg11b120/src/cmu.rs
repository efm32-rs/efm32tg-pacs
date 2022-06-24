#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - HFRCO Control Register"]
    pub hfrcoctrl: crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x18 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - LFRCO Control Register"]
    pub lfrcoctrl: crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>,
    #[doc = "0x24 - HFXO Control Register"]
    pub hfxoctrl: crate::Reg<hfxoctrl::HFXOCTRL_SPEC>,
    #[doc = "0x28 - HFXO Control 1"]
    pub hfxoctrl1: crate::Reg<hfxoctrl1::HFXOCTRL1_SPEC>,
    #[doc = "0x2c - HFXO Startup Control"]
    pub hfxostartupctrl: crate::Reg<hfxostartupctrl::HFXOSTARTUPCTRL_SPEC>,
    #[doc = "0x30 - HFXO Steady State Control"]
    pub hfxosteadystatectrl: crate::Reg<hfxosteadystatectrl::HFXOSTEADYSTATECTRL_SPEC>,
    #[doc = "0x34 - HFXO Timeout Control"]
    pub hfxotimeoutctrl: crate::Reg<hfxotimeoutctrl::HFXOTIMEOUTCTRL_SPEC>,
    #[doc = "0x38 - LFXO Control Register"]
    pub lfxoctrl: crate::Reg<lfxoctrl::LFXOCTRL_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - DPLL Control Register"]
    pub dpllctrl: crate::Reg<dpllctrl::DPLLCTRL_SPEC>,
    #[doc = "0x44 - DPLL Control Register"]
    pub dpllctrl1: crate::Reg<dpllctrl1::DPLLCTRL1_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x50 - Calibration Control Register"]
    pub calctrl: crate::Reg<calctrl::CALCTRL_SPEC>,
    #[doc = "0x54 - Calibration Counter Register"]
    pub calcnt: crate::Reg<calcnt::CALCNT_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x60 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: crate::Reg<oscencmd::OSCENCMD_SPEC>,
    #[doc = "0x64 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x70 - Debug Trace Clock Select"]
    pub dbgclksel: crate::Reg<dbgclksel::DBGCLKSEL_SPEC>,
    #[doc = "0x74 - High Frequency Clock Select Command Register"]
    pub hfclksel: crate::Reg<hfclksel::HFCLKSEL_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x80 - Low Frequency A Clock Select Register"]
    pub lfaclksel: crate::Reg<lfaclksel::LFACLKSEL_SPEC>,
    #[doc = "0x84 - Low Frequency B Clock Select Register"]
    pub lfbclksel: crate::Reg<lfbclksel::LFBCLKSEL_SPEC>,
    #[doc = "0x88 - Low Frequency E Clock Select Register"]
    pub lfeclksel: crate::Reg<lfeclksel::LFECLKSEL_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x90 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x94 - HFCLK Status Register"]
    pub hfclkstatus: crate::Reg<hfclkstatus::HFCLKSTATUS_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x9c - HFXO Trim Status"]
    pub hfxotrimstatus: crate::Reg<hfxotrimstatus::HFXOTRIMSTATUS_SPEC>,
    #[doc = "0xa0 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0xac - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0xb0 - High Frequency Bus Clock Enable Register 0"]
    pub hfbusclken0: crate::Reg<hfbusclken0::HFBUSCLKEN0_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0xc0 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>,
    #[doc = "0xc4 - High Frequency Peripheral Clock Enable Register 1"]
    pub hfperclken1: crate::Reg<hfperclken1::HFPERCLKEN1_SPEC>,
    _reserved31: [u8; 0x18],
    #[doc = "0xe0 - Low Frequency a Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: crate::Reg<lfaclken0::LFACLKEN0_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0xe8 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: crate::Reg<lfbclken0::LFBCLKEN0_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0xf0 - Low Frequency E Clock Enable Register 0 (Async Reg)"]
    pub lfeclken0: crate::Reg<lfeclken0::LFECLKEN0_SPEC>,
    _reserved34: [u8; 0x0c],
    #[doc = "0x100 - High Frequency Clock Prescaler Register"]
    pub hfpresc: crate::Reg<hfpresc::HFPRESC_SPEC>,
    #[doc = "0x104 - High Frequency Bus Clock Prescaler Register"]
    pub hfbuspresc: crate::Reg<hfbuspresc::HFBUSPRESC_SPEC>,
    #[doc = "0x108 - High Frequency Core Clock Prescaler Register"]
    pub hfcorepresc: crate::Reg<hfcorepresc::HFCOREPRESC_SPEC>,
    #[doc = "0x10c - High Frequency Peripheral Clock Prescaler Register"]
    pub hfperpresc: crate::Reg<hfperpresc::HFPERPRESC_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x114 - High Frequency Export Clock Prescaler Register"]
    pub hfexppresc: crate::Reg<hfexppresc::HFEXPPRESC_SPEC>,
    #[doc = "0x118 - High Frequency Peripheral Clock Prescaler B Register"]
    pub hfperprescb: crate::Reg<hfperprescb::HFPERPRESCB_SPEC>,
    #[doc = "0x11c - High Frequency Peripheral Clock Prescaler C Register"]
    pub hfperprescc: crate::Reg<hfperprescc::HFPERPRESCC_SPEC>,
    #[doc = "0x120 - Low Frequency a Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: crate::Reg<lfapresc0::LFAPRESC0_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x128 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: crate::Reg<lfbpresc0::LFBPRESC0_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0x130 - Low Frequency E Prescaler Register 0 (Async Reg)"]
    pub lfepresc0: crate::Reg<lfepresc0::LFEPRESC0_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0x140 - Synchronization Busy Register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x144 - Freeze Register"]
    pub freeze: crate::Reg<freeze::FREEZE_SPEC>,
    _reserved46: [u8; 0x08],
    #[doc = "0x150 - PCNT Control Register"]
    pub pcntctrl: crate::Reg<pcntctrl::PCNTCTRL_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0x15c - ADC Control Register"]
    pub adcctrl: crate::Reg<adcctrl::ADCCTRL_SPEC>,
    _reserved48: [u8; 0x10],
    #[doc = "0x170 - I/O Routing Pin Enable Register"]
    pub routepen: crate::Reg<routepen::ROUTEPEN_SPEC>,
    #[doc = "0x174 - I/O Routing Location Register"]
    pub routeloc0: crate::Reg<routeloc0::ROUTELOC0_SPEC>,
    #[doc = "0x178 - I/O Routing Location Register"]
    pub routeloc1: crate::Reg<routeloc1::ROUTELOC1_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x180 - Configuration Lock Register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x184 - HFRCO Spread Spectrum Register"]
    pub hfrcoss: crate::Reg<hfrcoss::HFRCOSS_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "HFRCOCTRL register accessor: an alias for `Reg<HFRCOCTRL_SPEC>`"]
pub type HFRCOCTRL = crate::Reg<hfrcoctrl::HFRCOCTRL_SPEC>;
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "AUXHFRCOCTRL register accessor: an alias for `Reg<AUXHFRCOCTRL_SPEC>`"]
pub type AUXHFRCOCTRL = crate::Reg<auxhfrcoctrl::AUXHFRCOCTRL_SPEC>;
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "LFRCOCTRL register accessor: an alias for `Reg<LFRCOCTRL_SPEC>`"]
pub type LFRCOCTRL = crate::Reg<lfrcoctrl::LFRCOCTRL_SPEC>;
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "HFXOCTRL register accessor: an alias for `Reg<HFXOCTRL_SPEC>`"]
pub type HFXOCTRL = crate::Reg<hfxoctrl::HFXOCTRL_SPEC>;
#[doc = "HFXO Control Register"]
pub mod hfxoctrl;
#[doc = "HFXOCTRL1 register accessor: an alias for `Reg<HFXOCTRL1_SPEC>`"]
pub type HFXOCTRL1 = crate::Reg<hfxoctrl1::HFXOCTRL1_SPEC>;
#[doc = "HFXO Control 1"]
pub mod hfxoctrl1;
#[doc = "HFXOSTARTUPCTRL register accessor: an alias for `Reg<HFXOSTARTUPCTRL_SPEC>`"]
pub type HFXOSTARTUPCTRL = crate::Reg<hfxostartupctrl::HFXOSTARTUPCTRL_SPEC>;
#[doc = "HFXO Startup Control"]
pub mod hfxostartupctrl;
#[doc = "HFXOSTEADYSTATECTRL register accessor: an alias for `Reg<HFXOSTEADYSTATECTRL_SPEC>`"]
pub type HFXOSTEADYSTATECTRL = crate::Reg<hfxosteadystatectrl::HFXOSTEADYSTATECTRL_SPEC>;
#[doc = "HFXO Steady State Control"]
pub mod hfxosteadystatectrl;
#[doc = "HFXOTIMEOUTCTRL register accessor: an alias for `Reg<HFXOTIMEOUTCTRL_SPEC>`"]
pub type HFXOTIMEOUTCTRL = crate::Reg<hfxotimeoutctrl::HFXOTIMEOUTCTRL_SPEC>;
#[doc = "HFXO Timeout Control"]
pub mod hfxotimeoutctrl;
#[doc = "LFXOCTRL register accessor: an alias for `Reg<LFXOCTRL_SPEC>`"]
pub type LFXOCTRL = crate::Reg<lfxoctrl::LFXOCTRL_SPEC>;
#[doc = "LFXO Control Register"]
pub mod lfxoctrl;
#[doc = "DPLLCTRL register accessor: an alias for `Reg<DPLLCTRL_SPEC>`"]
pub type DPLLCTRL = crate::Reg<dpllctrl::DPLLCTRL_SPEC>;
#[doc = "DPLL Control Register"]
pub mod dpllctrl;
#[doc = "DPLLCTRL1 register accessor: an alias for `Reg<DPLLCTRL1_SPEC>`"]
pub type DPLLCTRL1 = crate::Reg<dpllctrl1::DPLLCTRL1_SPEC>;
#[doc = "DPLL Control Register"]
pub mod dpllctrl1;
#[doc = "CALCTRL register accessor: an alias for `Reg<CALCTRL_SPEC>`"]
pub type CALCTRL = crate::Reg<calctrl::CALCTRL_SPEC>;
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "CALCNT register accessor: an alias for `Reg<CALCNT_SPEC>`"]
pub type CALCNT = crate::Reg<calcnt::CALCNT_SPEC>;
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "OSCENCMD register accessor: an alias for `Reg<OSCENCMD_SPEC>`"]
pub type OSCENCMD = crate::Reg<oscencmd::OSCENCMD_SPEC>;
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DBGCLKSEL register accessor: an alias for `Reg<DBGCLKSEL_SPEC>`"]
pub type DBGCLKSEL = crate::Reg<dbgclksel::DBGCLKSEL_SPEC>;
#[doc = "Debug Trace Clock Select"]
pub mod dbgclksel;
#[doc = "HFCLKSEL register accessor: an alias for `Reg<HFCLKSEL_SPEC>`"]
pub type HFCLKSEL = crate::Reg<hfclksel::HFCLKSEL_SPEC>;
#[doc = "High Frequency Clock Select Command Register"]
pub mod hfclksel;
#[doc = "LFACLKSEL register accessor: an alias for `Reg<LFACLKSEL_SPEC>`"]
pub type LFACLKSEL = crate::Reg<lfaclksel::LFACLKSEL_SPEC>;
#[doc = "Low Frequency A Clock Select Register"]
pub mod lfaclksel;
#[doc = "LFBCLKSEL register accessor: an alias for `Reg<LFBCLKSEL_SPEC>`"]
pub type LFBCLKSEL = crate::Reg<lfbclksel::LFBCLKSEL_SPEC>;
#[doc = "Low Frequency B Clock Select Register"]
pub mod lfbclksel;
#[doc = "LFECLKSEL register accessor: an alias for `Reg<LFECLKSEL_SPEC>`"]
pub type LFECLKSEL = crate::Reg<lfeclksel::LFECLKSEL_SPEC>;
#[doc = "Low Frequency E Clock Select Register"]
pub mod lfeclksel;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "HFCLKSTATUS register accessor: an alias for `Reg<HFCLKSTATUS_SPEC>`"]
pub type HFCLKSTATUS = crate::Reg<hfclkstatus::HFCLKSTATUS_SPEC>;
#[doc = "HFCLK Status Register"]
pub mod hfclkstatus;
#[doc = "HFXOTRIMSTATUS register accessor: an alias for `Reg<HFXOTRIMSTATUS_SPEC>`"]
pub type HFXOTRIMSTATUS = crate::Reg<hfxotrimstatus::HFXOTRIMSTATUS_SPEC>;
#[doc = "HFXO Trim Status"]
pub mod hfxotrimstatus;
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
#[doc = "HFBUSCLKEN0 register accessor: an alias for `Reg<HFBUSCLKEN0_SPEC>`"]
pub type HFBUSCLKEN0 = crate::Reg<hfbusclken0::HFBUSCLKEN0_SPEC>;
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub mod hfbusclken0;
#[doc = "HFPERCLKEN0 register accessor: an alias for `Reg<HFPERCLKEN0_SPEC>`"]
pub type HFPERCLKEN0 = crate::Reg<hfperclken0::HFPERCLKEN0_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "HFPERCLKEN1 register accessor: an alias for `Reg<HFPERCLKEN1_SPEC>`"]
pub type HFPERCLKEN1 = crate::Reg<hfperclken1::HFPERCLKEN1_SPEC>;
#[doc = "High Frequency Peripheral Clock Enable Register 1"]
pub mod hfperclken1;
#[doc = "LFACLKEN0 register accessor: an alias for `Reg<LFACLKEN0_SPEC>`"]
pub type LFACLKEN0 = crate::Reg<lfaclken0::LFACLKEN0_SPEC>;
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "LFBCLKEN0 register accessor: an alias for `Reg<LFBCLKEN0_SPEC>`"]
pub type LFBCLKEN0 = crate::Reg<lfbclken0::LFBCLKEN0_SPEC>;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "LFECLKEN0 register accessor: an alias for `Reg<LFECLKEN0_SPEC>`"]
pub type LFECLKEN0 = crate::Reg<lfeclken0::LFECLKEN0_SPEC>;
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub mod lfeclken0;
#[doc = "HFPRESC register accessor: an alias for `Reg<HFPRESC_SPEC>`"]
pub type HFPRESC = crate::Reg<hfpresc::HFPRESC_SPEC>;
#[doc = "High Frequency Clock Prescaler Register"]
pub mod hfpresc;
#[doc = "HFBUSPRESC register accessor: an alias for `Reg<HFBUSPRESC_SPEC>`"]
pub type HFBUSPRESC = crate::Reg<hfbuspresc::HFBUSPRESC_SPEC>;
#[doc = "High Frequency Bus Clock Prescaler Register"]
pub mod hfbuspresc;
#[doc = "HFCOREPRESC register accessor: an alias for `Reg<HFCOREPRESC_SPEC>`"]
pub type HFCOREPRESC = crate::Reg<hfcorepresc::HFCOREPRESC_SPEC>;
#[doc = "High Frequency Core Clock Prescaler Register"]
pub mod hfcorepresc;
#[doc = "HFPERPRESC register accessor: an alias for `Reg<HFPERPRESC_SPEC>`"]
pub type HFPERPRESC = crate::Reg<hfperpresc::HFPERPRESC_SPEC>;
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub mod hfperpresc;
#[doc = "HFEXPPRESC register accessor: an alias for `Reg<HFEXPPRESC_SPEC>`"]
pub type HFEXPPRESC = crate::Reg<hfexppresc::HFEXPPRESC_SPEC>;
#[doc = "High Frequency Export Clock Prescaler Register"]
pub mod hfexppresc;
#[doc = "HFPERPRESCB register accessor: an alias for `Reg<HFPERPRESCB_SPEC>`"]
pub type HFPERPRESCB = crate::Reg<hfperprescb::HFPERPRESCB_SPEC>;
#[doc = "High Frequency Peripheral Clock Prescaler B Register"]
pub mod hfperprescb;
#[doc = "HFPERPRESCC register accessor: an alias for `Reg<HFPERPRESCC_SPEC>`"]
pub type HFPERPRESCC = crate::Reg<hfperprescc::HFPERPRESCC_SPEC>;
#[doc = "High Frequency Peripheral Clock Prescaler C Register"]
pub mod hfperprescc;
#[doc = "LFAPRESC0 register accessor: an alias for `Reg<LFAPRESC0_SPEC>`"]
pub type LFAPRESC0 = crate::Reg<lfapresc0::LFAPRESC0_SPEC>;
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "LFBPRESC0 register accessor: an alias for `Reg<LFBPRESC0_SPEC>`"]
pub type LFBPRESC0 = crate::Reg<lfbpresc0::LFBPRESC0_SPEC>;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "LFEPRESC0 register accessor: an alias for `Reg<LFEPRESC0_SPEC>`"]
pub type LFEPRESC0 = crate::Reg<lfepresc0::LFEPRESC0_SPEC>;
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub mod lfepresc0;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FREEZE register accessor: an alias for `Reg<FREEZE_SPEC>`"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "PCNTCTRL register accessor: an alias for `Reg<PCNTCTRL_SPEC>`"]
pub type PCNTCTRL = crate::Reg<pcntctrl::PCNTCTRL_SPEC>;
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ADCCTRL register accessor: an alias for `Reg<ADCCTRL_SPEC>`"]
pub type ADCCTRL = crate::Reg<adcctrl::ADCCTRL_SPEC>;
#[doc = "ADC Control Register"]
pub mod adcctrl;
#[doc = "ROUTEPEN register accessor: an alias for `Reg<ROUTEPEN_SPEC>`"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 register accessor: an alias for `Reg<ROUTELOC0_SPEC>`"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 register accessor: an alias for `Reg<ROUTELOC1_SPEC>`"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "HFRCOSS register accessor: an alias for `Reg<HFRCOSS_SPEC>`"]
pub type HFRCOSS = crate::Reg<hfrcoss::HFRCOSS_SPEC>;
#[doc = "HFRCO Spread Spectrum Register"]
pub mod hfrcoss;
