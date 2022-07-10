#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    _reserved4: [u8; 0x24],
    #[doc = "0x40 - PPU Control Register"]
    pub ppuctrl: crate::Reg<ppuctrl::PPUCTRL_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50 - PPU Privilege Access Type Descriptor 0"]
    pub ppupatd0: crate::Reg<ppupatd0::PPUPATD0_SPEC>,
    #[doc = "0x54 - PPU Privilege Access Type Descriptor 1"]
    pub ppupatd1: crate::Reg<ppupatd1::PPUPATD1_SPEC>,
    _reserved7: [u8; 0x38],
    #[doc = "0x90 - PPU Fault Status"]
    pub ppufs: crate::Reg<ppufs::PPUFS_SPEC>,
}
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
#[doc = "PPUCTRL register accessor: an alias for `Reg<PPUCTRL_SPEC>`"]
pub type PPUCTRL = crate::Reg<ppuctrl::PPUCTRL_SPEC>;
#[doc = "PPU Control Register"]
pub mod ppuctrl;
#[doc = "PPUPATD0 register accessor: an alias for `Reg<PPUPATD0_SPEC>`"]
pub type PPUPATD0 = crate::Reg<ppupatd0::PPUPATD0_SPEC>;
#[doc = "PPU Privilege Access Type Descriptor 0"]
pub mod ppupatd0;
#[doc = "PPUPATD1 register accessor: an alias for `Reg<PPUPATD1_SPEC>`"]
pub type PPUPATD1 = crate::Reg<ppupatd1::PPUPATD1_SPEC>;
#[doc = "PPU Privilege Access Type Descriptor 1"]
pub mod ppupatd1;
#[doc = "PPUFS register accessor: an alias for `Reg<PPUFS_SPEC>`"]
pub type PPUFS = crate::Reg<ppufs::PPUFS_SPEC>;
#[doc = "PPU Fault Status"]
pub mod ppufs;
