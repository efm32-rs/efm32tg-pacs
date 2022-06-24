#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Error Count Register"]
    pub errcnt: crate::Reg<errcnt::ERRCNT_SPEC>,
    #[doc = "0x0c - Bit Timing Register"]
    pub bittiming: crate::Reg<bittiming::BITTIMING_SPEC>,
    #[doc = "0x10 - Interrupt Identification Register"]
    pub intid: crate::Reg<intid::INTID_SPEC>,
    #[doc = "0x14 - Test Register"]
    pub test: crate::Reg<test::TEST_SPEC>,
    #[doc = "0x18 - BRP Extension Register"]
    pub brpe: crate::Reg<brpe::BRPE_SPEC>,
    #[doc = "0x1c - Transmission Request Register"]
    pub transreq: crate::Reg<transreq::TRANSREQ_SPEC>,
    #[doc = "0x20 - New Data Register"]
    pub messagedata: crate::Reg<messagedata::MESSAGEDATA_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Message Valid Register"]
    pub messagestate: crate::Reg<messagestate::MESSAGESTATE_SPEC>,
    #[doc = "0x2c - Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x30 - Message Object Interrupt Flag Register"]
    pub if0if: crate::Reg<if0if::IF0IF_SPEC>,
    #[doc = "0x34 - Message Object Interrupt Flag Set Register"]
    pub if0ifs: crate::Reg<if0ifs::IF0IFS_SPEC>,
    #[doc = "0x38 - Message Object Interrupt Flag Clear Register"]
    pub if0ifc: crate::Reg<if0ifc::IF0IFC_SPEC>,
    #[doc = "0x3c - Message Object Interrupt Enable Register"]
    pub if0ien: crate::Reg<if0ien::IF0IEN_SPEC>,
    #[doc = "0x40 - Status Interrupt Flag Register"]
    pub if1if: crate::Reg<if1if::IF1IF_SPEC>,
    #[doc = "0x44 - Message Object Interrupt Flag Set Register"]
    pub if1ifs: crate::Reg<if1ifs::IF1IFS_SPEC>,
    #[doc = "0x48 - Message Object Interrupt Flag Clear Register"]
    pub if1ifc: crate::Reg<if1ifc::IF1IFC_SPEC>,
    #[doc = "0x4c - Status Interrupt Enable Register"]
    pub if1ien: crate::Reg<if1ien::IF1IEN_SPEC>,
    #[doc = "0x50 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x60 - Interface Command Mask Register"]
    pub mir0_cmdmask: crate::Reg<mir0_cmdmask::MIR0_CMDMASK_SPEC>,
    #[doc = "0x64 - Interface Mask Register"]
    pub mir0_mask: crate::Reg<mir0_mask::MIR0_MASK_SPEC>,
    #[doc = "0x68 - Interface Arbitration Register"]
    pub mir0_arb: crate::Reg<mir0_arb::MIR0_ARB_SPEC>,
    #[doc = "0x6c - Interface Message Control Register"]
    pub mir0_ctrl: crate::Reg<mir0_ctrl::MIR0_CTRL_SPEC>,
    #[doc = "0x70 - Interface Data a Register"]
    pub mir0_datal: crate::Reg<mir0_datal::MIR0_DATAL_SPEC>,
    #[doc = "0x74 - Interface Data B Register"]
    pub mir0_datah: crate::Reg<mir0_datah::MIR0_DATAH_SPEC>,
    #[doc = "0x78 - Interface Command Request Register"]
    pub mir0_cmdreq: crate::Reg<mir0_cmdreq::MIR0_CMDREQ_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x80 - Interface Command Mask Register"]
    pub mir1_cmdmask: crate::Reg<mir1_cmdmask::MIR1_CMDMASK_SPEC>,
    #[doc = "0x84 - Interface Mask Register"]
    pub mir1_mask: crate::Reg<mir1_mask::MIR1_MASK_SPEC>,
    #[doc = "0x88 - Interface Arbitration Register"]
    pub mir1_arb: crate::Reg<mir1_arb::MIR1_ARB_SPEC>,
    #[doc = "0x8c - Interface Message Control Register"]
    pub mir1_ctrl: crate::Reg<mir1_ctrl::MIR1_CTRL_SPEC>,
    #[doc = "0x90 - Interface Data a Register"]
    pub mir1_datal: crate::Reg<mir1_datal::MIR1_DATAL_SPEC>,
    #[doc = "0x94 - Interface Data B Register"]
    pub mir1_datah: crate::Reg<mir1_datah::MIR1_DATAH_SPEC>,
    #[doc = "0x98 - Interface Command Request Register"]
    pub mir1_cmdreq: crate::Reg<mir1_cmdreq::MIR1_CMDREQ_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ERRCNT register accessor: an alias for `Reg<ERRCNT_SPEC>`"]
pub type ERRCNT = crate::Reg<errcnt::ERRCNT_SPEC>;
#[doc = "Error Count Register"]
pub mod errcnt;
#[doc = "BITTIMING register accessor: an alias for `Reg<BITTIMING_SPEC>`"]
pub type BITTIMING = crate::Reg<bittiming::BITTIMING_SPEC>;
#[doc = "Bit Timing Register"]
pub mod bittiming;
#[doc = "INTID register accessor: an alias for `Reg<INTID_SPEC>`"]
pub type INTID = crate::Reg<intid::INTID_SPEC>;
#[doc = "Interrupt Identification Register"]
pub mod intid;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "BRPE register accessor: an alias for `Reg<BRPE_SPEC>`"]
pub type BRPE = crate::Reg<brpe::BRPE_SPEC>;
#[doc = "BRP Extension Register"]
pub mod brpe;
#[doc = "TRANSREQ register accessor: an alias for `Reg<TRANSREQ_SPEC>`"]
pub type TRANSREQ = crate::Reg<transreq::TRANSREQ_SPEC>;
#[doc = "Transmission Request Register"]
pub mod transreq;
#[doc = "MESSAGEDATA register accessor: an alias for `Reg<MESSAGEDATA_SPEC>`"]
pub type MESSAGEDATA = crate::Reg<messagedata::MESSAGEDATA_SPEC>;
#[doc = "New Data Register"]
pub mod messagedata;
#[doc = "MESSAGESTATE register accessor: an alias for `Reg<MESSAGESTATE_SPEC>`"]
pub type MESSAGESTATE = crate::Reg<messagestate::MESSAGESTATE_SPEC>;
#[doc = "Message Valid Register"]
pub mod messagestate;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "IF0IF register accessor: an alias for `Reg<IF0IF_SPEC>`"]
pub type IF0IF = crate::Reg<if0if::IF0IF_SPEC>;
#[doc = "Message Object Interrupt Flag Register"]
pub mod if0if;
#[doc = "IF0IFS register accessor: an alias for `Reg<IF0IFS_SPEC>`"]
pub type IF0IFS = crate::Reg<if0ifs::IF0IFS_SPEC>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if0ifs;
#[doc = "IF0IFC register accessor: an alias for `Reg<IF0IFC_SPEC>`"]
pub type IF0IFC = crate::Reg<if0ifc::IF0IFC_SPEC>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if0ifc;
#[doc = "IF0IEN register accessor: an alias for `Reg<IF0IEN_SPEC>`"]
pub type IF0IEN = crate::Reg<if0ien::IF0IEN_SPEC>;
#[doc = "Message Object Interrupt Enable Register"]
pub mod if0ien;
#[doc = "IF1IF register accessor: an alias for `Reg<IF1IF_SPEC>`"]
pub type IF1IF = crate::Reg<if1if::IF1IF_SPEC>;
#[doc = "Status Interrupt Flag Register"]
pub mod if1if;
#[doc = "IF1IFS register accessor: an alias for `Reg<IF1IFS_SPEC>`"]
pub type IF1IFS = crate::Reg<if1ifs::IF1IFS_SPEC>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if1ifs;
#[doc = "IF1IFC register accessor: an alias for `Reg<IF1IFC_SPEC>`"]
pub type IF1IFC = crate::Reg<if1ifc::IF1IFC_SPEC>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if1ifc;
#[doc = "IF1IEN register accessor: an alias for `Reg<IF1IEN_SPEC>`"]
pub type IF1IEN = crate::Reg<if1ien::IF1IEN_SPEC>;
#[doc = "Status Interrupt Enable Register"]
pub mod if1ien;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "MIR0_CMDMASK register accessor: an alias for `Reg<MIR0_CMDMASK_SPEC>`"]
pub type MIR0_CMDMASK = crate::Reg<mir0_cmdmask::MIR0_CMDMASK_SPEC>;
#[doc = "Interface Command Mask Register"]
pub mod mir0_cmdmask;
#[doc = "MIR0_MASK register accessor: an alias for `Reg<MIR0_MASK_SPEC>`"]
pub type MIR0_MASK = crate::Reg<mir0_mask::MIR0_MASK_SPEC>;
#[doc = "Interface Mask Register"]
pub mod mir0_mask;
#[doc = "MIR0_ARB register accessor: an alias for `Reg<MIR0_ARB_SPEC>`"]
pub type MIR0_ARB = crate::Reg<mir0_arb::MIR0_ARB_SPEC>;
#[doc = "Interface Arbitration Register"]
pub mod mir0_arb;
#[doc = "MIR0_CTRL register accessor: an alias for `Reg<MIR0_CTRL_SPEC>`"]
pub type MIR0_CTRL = crate::Reg<mir0_ctrl::MIR0_CTRL_SPEC>;
#[doc = "Interface Message Control Register"]
pub mod mir0_ctrl;
#[doc = "MIR0_DATAL register accessor: an alias for `Reg<MIR0_DATAL_SPEC>`"]
pub type MIR0_DATAL = crate::Reg<mir0_datal::MIR0_DATAL_SPEC>;
#[doc = "Interface Data a Register"]
pub mod mir0_datal;
#[doc = "MIR0_DATAH register accessor: an alias for `Reg<MIR0_DATAH_SPEC>`"]
pub type MIR0_DATAH = crate::Reg<mir0_datah::MIR0_DATAH_SPEC>;
#[doc = "Interface Data B Register"]
pub mod mir0_datah;
#[doc = "MIR0_CMDREQ register accessor: an alias for `Reg<MIR0_CMDREQ_SPEC>`"]
pub type MIR0_CMDREQ = crate::Reg<mir0_cmdreq::MIR0_CMDREQ_SPEC>;
#[doc = "Interface Command Request Register"]
pub mod mir0_cmdreq;
#[doc = "MIR1_CMDMASK register accessor: an alias for `Reg<MIR1_CMDMASK_SPEC>`"]
pub type MIR1_CMDMASK = crate::Reg<mir1_cmdmask::MIR1_CMDMASK_SPEC>;
#[doc = "Interface Command Mask Register"]
pub mod mir1_cmdmask;
#[doc = "MIR1_MASK register accessor: an alias for `Reg<MIR1_MASK_SPEC>`"]
pub type MIR1_MASK = crate::Reg<mir1_mask::MIR1_MASK_SPEC>;
#[doc = "Interface Mask Register"]
pub mod mir1_mask;
#[doc = "MIR1_ARB register accessor: an alias for `Reg<MIR1_ARB_SPEC>`"]
pub type MIR1_ARB = crate::Reg<mir1_arb::MIR1_ARB_SPEC>;
#[doc = "Interface Arbitration Register"]
pub mod mir1_arb;
#[doc = "MIR1_CTRL register accessor: an alias for `Reg<MIR1_CTRL_SPEC>`"]
pub type MIR1_CTRL = crate::Reg<mir1_ctrl::MIR1_CTRL_SPEC>;
#[doc = "Interface Message Control Register"]
pub mod mir1_ctrl;
#[doc = "MIR1_DATAL register accessor: an alias for `Reg<MIR1_DATAL_SPEC>`"]
pub type MIR1_DATAL = crate::Reg<mir1_datal::MIR1_DATAL_SPEC>;
#[doc = "Interface Data a Register"]
pub mod mir1_datal;
#[doc = "MIR1_DATAH register accessor: an alias for `Reg<MIR1_DATAH_SPEC>`"]
pub type MIR1_DATAH = crate::Reg<mir1_datah::MIR1_DATAH_SPEC>;
#[doc = "Interface Data B Register"]
pub mod mir1_datah;
#[doc = "MIR1_CMDREQ register accessor: an alias for `Reg<MIR1_CMDREQ_SPEC>`"]
pub type MIR1_CMDREQ = crate::Reg<mir1_cmdreq::MIR1_CMDREQ_SPEC>;
#[doc = "Interface Command Request Register"]
pub mod mir1_cmdreq;
