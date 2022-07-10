#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - DMA Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x24 - DMA Channel Busy Register"]
    pub chbusy: crate::Reg<chbusy::CHBUSY_SPEC>,
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    pub chdone: crate::Reg<chdone::CHDONE_SPEC>,
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    pub dbghalt: crate::Reg<dbghalt::DBGHALT_SPEC>,
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    pub swreq: crate::Reg<swreq::SWREQ_SPEC>,
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    pub reqdis: crate::Reg<reqdis::REQDIS_SPEC>,
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    pub reqpend: crate::Reg<reqpend::REQPEND_SPEC>,
    #[doc = "0x3c - DMA Channel Link Load Register"]
    pub linkload: crate::Reg<linkload::LINKLOAD_SPEC>,
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    pub reqclear: crate::Reg<reqclear::REQCLEAR_SPEC>,
    _reserved12: [u8; 0x1c],
    #[doc = "0x60 - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x64 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x6c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    pub ch0_reqsel: crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>,
    #[doc = "0x84 - Channel Configuration Register"]
    pub ch0_cfg: crate::Reg<ch0_cfg::CH0_CFG_SPEC>,
    #[doc = "0x88 - Channel Loop Counter Register"]
    pub ch0_loop: crate::Reg<ch0_loop::CH0_LOOP_SPEC>,
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    pub ch0_src: crate::Reg<ch0_src::CH0_SRC_SPEC>,
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    pub ch0_dst: crate::Reg<ch0_dst::CH0_DST_SPEC>,
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    pub ch0_link: crate::Reg<ch0_link::CH0_LINK_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    pub ch1_reqsel: crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>,
    #[doc = "0xb4 - Channel Configuration Register"]
    pub ch1_cfg: crate::Reg<ch1_cfg::CH1_CFG_SPEC>,
    #[doc = "0xb8 - Channel Loop Counter Register"]
    pub ch1_loop: crate::Reg<ch1_loop::CH1_LOOP_SPEC>,
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    pub ch1_src: crate::Reg<ch1_src::CH1_SRC_SPEC>,
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    pub ch1_dst: crate::Reg<ch1_dst::CH1_DST_SPEC>,
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    pub ch1_link: crate::Reg<ch1_link::CH1_LINK_SPEC>,
    _reserved30: [u8; 0x14],
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    pub ch2_reqsel: crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>,
    #[doc = "0xe4 - Channel Configuration Register"]
    pub ch2_cfg: crate::Reg<ch2_cfg::CH2_CFG_SPEC>,
    #[doc = "0xe8 - Channel Loop Counter Register"]
    pub ch2_loop: crate::Reg<ch2_loop::CH2_LOOP_SPEC>,
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    pub ch2_src: crate::Reg<ch2_src::CH2_SRC_SPEC>,
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    pub ch2_dst: crate::Reg<ch2_dst::CH2_DST_SPEC>,
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    pub ch2_link: crate::Reg<ch2_link::CH2_LINK_SPEC>,
    _reserved37: [u8; 0x14],
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    pub ch3_reqsel: crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>,
    #[doc = "0x114 - Channel Configuration Register"]
    pub ch3_cfg: crate::Reg<ch3_cfg::CH3_CFG_SPEC>,
    #[doc = "0x118 - Channel Loop Counter Register"]
    pub ch3_loop: crate::Reg<ch3_loop::CH3_LOOP_SPEC>,
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    pub ch3_src: crate::Reg<ch3_src::CH3_SRC_SPEC>,
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    pub ch3_dst: crate::Reg<ch3_dst::CH3_DST_SPEC>,
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    pub ch3_link: crate::Reg<ch3_link::CH3_LINK_SPEC>,
    _reserved44: [u8; 0x14],
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    pub ch4_reqsel: crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>,
    #[doc = "0x144 - Channel Configuration Register"]
    pub ch4_cfg: crate::Reg<ch4_cfg::CH4_CFG_SPEC>,
    #[doc = "0x148 - Channel Loop Counter Register"]
    pub ch4_loop: crate::Reg<ch4_loop::CH4_LOOP_SPEC>,
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    pub ch4_src: crate::Reg<ch4_src::CH4_SRC_SPEC>,
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    pub ch4_dst: crate::Reg<ch4_dst::CH4_DST_SPEC>,
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    pub ch4_link: crate::Reg<ch4_link::CH4_LINK_SPEC>,
    _reserved51: [u8; 0x14],
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    pub ch5_reqsel: crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>,
    #[doc = "0x174 - Channel Configuration Register"]
    pub ch5_cfg: crate::Reg<ch5_cfg::CH5_CFG_SPEC>,
    #[doc = "0x178 - Channel Loop Counter Register"]
    pub ch5_loop: crate::Reg<ch5_loop::CH5_LOOP_SPEC>,
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    pub ch5_src: crate::Reg<ch5_src::CH5_SRC_SPEC>,
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    pub ch5_dst: crate::Reg<ch5_dst::CH5_DST_SPEC>,
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    pub ch5_link: crate::Reg<ch5_link::CH5_LINK_SPEC>,
    _reserved58: [u8; 0x14],
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    pub ch6_reqsel: crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>,
    #[doc = "0x1a4 - Channel Configuration Register"]
    pub ch6_cfg: crate::Reg<ch6_cfg::CH6_CFG_SPEC>,
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    pub ch6_loop: crate::Reg<ch6_loop::CH6_LOOP_SPEC>,
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    pub ch6_ctrl: crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>,
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    pub ch6_src: crate::Reg<ch6_src::CH6_SRC_SPEC>,
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    pub ch6_dst: crate::Reg<ch6_dst::CH6_DST_SPEC>,
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    pub ch6_link: crate::Reg<ch6_link::CH6_LINK_SPEC>,
    _reserved65: [u8; 0x14],
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    pub ch7_reqsel: crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>,
    #[doc = "0x1d4 - Channel Configuration Register"]
    pub ch7_cfg: crate::Reg<ch7_cfg::CH7_CFG_SPEC>,
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    pub ch7_loop: crate::Reg<ch7_loop::CH7_LOOP_SPEC>,
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    pub ch7_ctrl: crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>,
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    pub ch7_src: crate::Reg<ch7_src::CH7_SRC_SPEC>,
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    pub ch7_dst: crate::Reg<ch7_dst::CH7_DST_SPEC>,
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    pub ch7_link: crate::Reg<ch7_link::CH7_LINK_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "CHBUSY register accessor: an alias for `Reg<CHBUSY_SPEC>`"]
pub type CHBUSY = crate::Reg<chbusy::CHBUSY_SPEC>;
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE register accessor: an alias for `Reg<CHDONE_SPEC>`"]
pub type CHDONE = crate::Reg<chdone::CHDONE_SPEC>;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DBGHALT register accessor: an alias for `Reg<DBGHALT_SPEC>`"]
pub type DBGHALT = crate::Reg<dbghalt::DBGHALT_SPEC>;
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ register accessor: an alias for `Reg<SWREQ_SPEC>`"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "REQDIS register accessor: an alias for `Reg<REQDIS_SPEC>`"]
pub type REQDIS = crate::Reg<reqdis::REQDIS_SPEC>;
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "REQPEND register accessor: an alias for `Reg<REQPEND_SPEC>`"]
pub type REQPEND = crate::Reg<reqpend::REQPEND_SPEC>;
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD register accessor: an alias for `Reg<LINKLOAD_SPEC>`"]
pub type LINKLOAD = crate::Reg<linkload::LINKLOAD_SPEC>;
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "REQCLEAR register accessor: an alias for `Reg<REQCLEAR_SPEC>`"]
pub type REQCLEAR = crate::Reg<reqclear::REQCLEAR_SPEC>;
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "CH0_REQSEL register accessor: an alias for `Reg<CH0_REQSEL_SPEC>`"]
pub type CH0_REQSEL = crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "CH0_CFG register accessor: an alias for `Reg<CH0_CFG_SPEC>`"]
pub type CH0_CFG = crate::Reg<ch0_cfg::CH0_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP register accessor: an alias for `Reg<CH0_LOOP_SPEC>`"]
pub type CH0_LOOP = crate::Reg<ch0_loop::CH0_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC register accessor: an alias for `Reg<CH0_SRC_SPEC>`"]
pub type CH0_SRC = crate::Reg<ch0_src::CH0_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "CH0_DST register accessor: an alias for `Reg<CH0_DST_SPEC>`"]
pub type CH0_DST = crate::Reg<ch0_dst::CH0_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "CH0_LINK register accessor: an alias for `Reg<CH0_LINK_SPEC>`"]
pub type CH0_LINK = crate::Reg<ch0_link::CH0_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "CH1_REQSEL register accessor: an alias for `Reg<CH1_REQSEL_SPEC>`"]
pub type CH1_REQSEL = crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "CH1_CFG register accessor: an alias for `Reg<CH1_CFG_SPEC>`"]
pub type CH1_CFG = crate::Reg<ch1_cfg::CH1_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP register accessor: an alias for `Reg<CH1_LOOP_SPEC>`"]
pub type CH1_LOOP = crate::Reg<ch1_loop::CH1_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC register accessor: an alias for `Reg<CH1_SRC_SPEC>`"]
pub type CH1_SRC = crate::Reg<ch1_src::CH1_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "CH1_DST register accessor: an alias for `Reg<CH1_DST_SPEC>`"]
pub type CH1_DST = crate::Reg<ch1_dst::CH1_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "CH1_LINK register accessor: an alias for `Reg<CH1_LINK_SPEC>`"]
pub type CH1_LINK = crate::Reg<ch1_link::CH1_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "CH2_REQSEL register accessor: an alias for `Reg<CH2_REQSEL_SPEC>`"]
pub type CH2_REQSEL = crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "CH2_CFG register accessor: an alias for `Reg<CH2_CFG_SPEC>`"]
pub type CH2_CFG = crate::Reg<ch2_cfg::CH2_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP register accessor: an alias for `Reg<CH2_LOOP_SPEC>`"]
pub type CH2_LOOP = crate::Reg<ch2_loop::CH2_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC register accessor: an alias for `Reg<CH2_SRC_SPEC>`"]
pub type CH2_SRC = crate::Reg<ch2_src::CH2_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "CH2_DST register accessor: an alias for `Reg<CH2_DST_SPEC>`"]
pub type CH2_DST = crate::Reg<ch2_dst::CH2_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "CH2_LINK register accessor: an alias for `Reg<CH2_LINK_SPEC>`"]
pub type CH2_LINK = crate::Reg<ch2_link::CH2_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "CH3_REQSEL register accessor: an alias for `Reg<CH3_REQSEL_SPEC>`"]
pub type CH3_REQSEL = crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "CH3_CFG register accessor: an alias for `Reg<CH3_CFG_SPEC>`"]
pub type CH3_CFG = crate::Reg<ch3_cfg::CH3_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP register accessor: an alias for `Reg<CH3_LOOP_SPEC>`"]
pub type CH3_LOOP = crate::Reg<ch3_loop::CH3_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC register accessor: an alias for `Reg<CH3_SRC_SPEC>`"]
pub type CH3_SRC = crate::Reg<ch3_src::CH3_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "CH3_DST register accessor: an alias for `Reg<CH3_DST_SPEC>`"]
pub type CH3_DST = crate::Reg<ch3_dst::CH3_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "CH3_LINK register accessor: an alias for `Reg<CH3_LINK_SPEC>`"]
pub type CH3_LINK = crate::Reg<ch3_link::CH3_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "CH4_REQSEL register accessor: an alias for `Reg<CH4_REQSEL_SPEC>`"]
pub type CH4_REQSEL = crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "CH4_CFG register accessor: an alias for `Reg<CH4_CFG_SPEC>`"]
pub type CH4_CFG = crate::Reg<ch4_cfg::CH4_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP register accessor: an alias for `Reg<CH4_LOOP_SPEC>`"]
pub type CH4_LOOP = crate::Reg<ch4_loop::CH4_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC register accessor: an alias for `Reg<CH4_SRC_SPEC>`"]
pub type CH4_SRC = crate::Reg<ch4_src::CH4_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "CH4_DST register accessor: an alias for `Reg<CH4_DST_SPEC>`"]
pub type CH4_DST = crate::Reg<ch4_dst::CH4_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "CH4_LINK register accessor: an alias for `Reg<CH4_LINK_SPEC>`"]
pub type CH4_LINK = crate::Reg<ch4_link::CH4_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "CH5_REQSEL register accessor: an alias for `Reg<CH5_REQSEL_SPEC>`"]
pub type CH5_REQSEL = crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "CH5_CFG register accessor: an alias for `Reg<CH5_CFG_SPEC>`"]
pub type CH5_CFG = crate::Reg<ch5_cfg::CH5_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP register accessor: an alias for `Reg<CH5_LOOP_SPEC>`"]
pub type CH5_LOOP = crate::Reg<ch5_loop::CH5_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC register accessor: an alias for `Reg<CH5_SRC_SPEC>`"]
pub type CH5_SRC = crate::Reg<ch5_src::CH5_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "CH5_DST register accessor: an alias for `Reg<CH5_DST_SPEC>`"]
pub type CH5_DST = crate::Reg<ch5_dst::CH5_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "CH5_LINK register accessor: an alias for `Reg<CH5_LINK_SPEC>`"]
pub type CH5_LINK = crate::Reg<ch5_link::CH5_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "CH6_REQSEL register accessor: an alias for `Reg<CH6_REQSEL_SPEC>`"]
pub type CH6_REQSEL = crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "CH6_CFG register accessor: an alias for `Reg<CH6_CFG_SPEC>`"]
pub type CH6_CFG = crate::Reg<ch6_cfg::CH6_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP register accessor: an alias for `Reg<CH6_LOOP_SPEC>`"]
pub type CH6_LOOP = crate::Reg<ch6_loop::CH6_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "CH6_CTRL register accessor: an alias for `Reg<CH6_CTRL_SPEC>`"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC register accessor: an alias for `Reg<CH6_SRC_SPEC>`"]
pub type CH6_SRC = crate::Reg<ch6_src::CH6_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "CH6_DST register accessor: an alias for `Reg<CH6_DST_SPEC>`"]
pub type CH6_DST = crate::Reg<ch6_dst::CH6_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "CH6_LINK register accessor: an alias for `Reg<CH6_LINK_SPEC>`"]
pub type CH6_LINK = crate::Reg<ch6_link::CH6_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "CH7_REQSEL register accessor: an alias for `Reg<CH7_REQSEL_SPEC>`"]
pub type CH7_REQSEL = crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "CH7_CFG register accessor: an alias for `Reg<CH7_CFG_SPEC>`"]
pub type CH7_CFG = crate::Reg<ch7_cfg::CH7_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP register accessor: an alias for `Reg<CH7_LOOP_SPEC>`"]
pub type CH7_LOOP = crate::Reg<ch7_loop::CH7_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "CH7_CTRL register accessor: an alias for `Reg<CH7_CTRL_SPEC>`"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC register accessor: an alias for `Reg<CH7_SRC_SPEC>`"]
pub type CH7_SRC = crate::Reg<ch7_src::CH7_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "CH7_DST register accessor: an alias for `Reg<CH7_DST_SPEC>`"]
pub type CH7_DST = crate::Reg<ch7_dst::CH7_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "CH7_LINK register accessor: an alias for `Reg<CH7_LINK_SPEC>`"]
pub type CH7_LINK = crate::Reg<ch7_link::CH7_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
