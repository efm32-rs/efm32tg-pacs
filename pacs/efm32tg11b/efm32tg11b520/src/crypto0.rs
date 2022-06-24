#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Wide Arithmetic Configuration"]
    pub wac: crate::Reg<wac::WAC_SPEC>,
    #[doc = "0x08 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14 - Data Status Register"]
    pub dstatus: crate::Reg<dstatus::DSTATUS_SPEC>,
    #[doc = "0x18 - Control Status Register"]
    pub cstatus: crate::Reg<cstatus::CSTATUS_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - KEY Register Access"]
    pub key: crate::Reg<key::KEY_SPEC>,
    #[doc = "0x24 - KEY Buffer Register Access"]
    pub keybuf: crate::Reg<keybuf::KEYBUF_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Sequence Control"]
    pub seqctrl: crate::Reg<seqctrl::SEQCTRL_SPEC>,
    #[doc = "0x34 - Sequence Control B"]
    pub seqctrlb: crate::Reg<seqctrlb::SEQCTRLB_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - AES Interrupt Flags"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: crate::Reg<ifs::IFS_SPEC>,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: crate::Reg<ifc::IFC_SPEC>,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x50 - Sequence Register 0"]
    pub seq0: crate::Reg<seq0::SEQ0_SPEC>,
    #[doc = "0x54 - Sequence Register 1"]
    pub seq1: crate::Reg<seq1::SEQ1_SPEC>,
    #[doc = "0x58 - Sequence Register 2"]
    pub seq2: crate::Reg<seq2::SEQ2_SPEC>,
    #[doc = "0x5c - Sequence Register 3"]
    pub seq3: crate::Reg<seq3::SEQ3_SPEC>,
    #[doc = "0x60 - Sequence Register 4"]
    pub seq4: crate::Reg<seq4::SEQ4_SPEC>,
    _reserved19: [u8; 0x1c],
    #[doc = "0x80 - DATA0 Register Access"]
    pub data0: crate::Reg<data0::DATA0_SPEC>,
    #[doc = "0x84 - DATA1 Register Access"]
    pub data1: crate::Reg<data1::DATA1_SPEC>,
    #[doc = "0x88 - DATA2 Register Access"]
    pub data2: crate::Reg<data2::DATA2_SPEC>,
    #[doc = "0x8c - DATA3 Register Access"]
    pub data3: crate::Reg<data3::DATA3_SPEC>,
    _reserved23: [u8; 0x10],
    #[doc = "0xa0 - DATA0XOR Register Access"]
    pub data0xor: crate::Reg<data0xor::DATA0XOR_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0xb0 - DATA0 Register Byte Access"]
    pub data0byte: crate::Reg<data0byte::DATA0BYTE_SPEC>,
    #[doc = "0xb4 - DATA1 Register Byte Access"]
    pub data1byte: crate::Reg<data1byte::DATA1BYTE_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xbc - DATA0 Register Byte XOR Access"]
    pub data0xorbyte: crate::Reg<data0xorbyte::DATA0XORBYTE_SPEC>,
    #[doc = "0xc0 - DATA0 Register Byte 12 Access"]
    pub data0byte12: crate::Reg<data0byte12::DATA0BYTE12_SPEC>,
    #[doc = "0xc4 - DATA0 Register Byte 13 Access"]
    pub data0byte13: crate::Reg<data0byte13::DATA0BYTE13_SPEC>,
    #[doc = "0xc8 - DATA0 Register Byte 14 Access"]
    pub data0byte14: crate::Reg<data0byte14::DATA0BYTE14_SPEC>,
    #[doc = "0xcc - DATA0 Register Byte 15 Access"]
    pub data0byte15: crate::Reg<data0byte15::DATA0BYTE15_SPEC>,
    _reserved31: [u8; 0x30],
    #[doc = "0x100 - DDATA0 Register Access"]
    pub ddata0: crate::Reg<ddata0::DDATA0_SPEC>,
    #[doc = "0x104 - DDATA1 Register Access"]
    pub ddata1: crate::Reg<ddata1::DDATA1_SPEC>,
    #[doc = "0x108 - DDATA2 Register Access"]
    pub ddata2: crate::Reg<ddata2::DDATA2_SPEC>,
    #[doc = "0x10c - DDATA3 Register Access"]
    pub ddata3: crate::Reg<ddata3::DDATA3_SPEC>,
    #[doc = "0x110 - DDATA4 Register Access"]
    pub ddata4: crate::Reg<ddata4::DDATA4_SPEC>,
    _reserved36: [u8; 0x1c],
    #[doc = "0x130 - DDATA0 Register Big Endian Access"]
    pub ddata0big: crate::Reg<ddata0big::DDATA0BIG_SPEC>,
    _reserved37: [u8; 0x0c],
    #[doc = "0x140 - DDATA0 Register Byte Access"]
    pub ddata0byte: crate::Reg<ddata0byte::DDATA0BYTE_SPEC>,
    #[doc = "0x144 - DDATA1 Register Byte Access"]
    pub ddata1byte: crate::Reg<ddata1byte::DDATA1BYTE_SPEC>,
    #[doc = "0x148 - DDATA0 Register Byte 32 Access"]
    pub ddata0byte32: crate::Reg<ddata0byte32::DDATA0BYTE32_SPEC>,
    _reserved40: [u8; 0x34],
    #[doc = "0x180 - QDATA0 Register Access"]
    pub qdata0: crate::Reg<qdata0::QDATA0_SPEC>,
    #[doc = "0x184 - QDATA1 Register Access"]
    pub qdata1: crate::Reg<qdata1::QDATA1_SPEC>,
    _reserved42: [u8; 0x1c],
    #[doc = "0x1a4 - QDATA1 Register Big Endian Access"]
    pub qdata1big: crate::Reg<qdata1big::QDATA1BIG_SPEC>,
    _reserved43: [u8; 0x18],
    #[doc = "0x1c0 - QDATA0 Register Byte Access"]
    pub qdata0byte: crate::Reg<qdata0byte::QDATA0BYTE_SPEC>,
    #[doc = "0x1c4 - QDATA1 Register Byte Access"]
    pub qdata1byte: crate::Reg<qdata1byte::QDATA1BYTE_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "WAC register accessor: an alias for `Reg<WAC_SPEC>`"]
pub type WAC = crate::Reg<wac::WAC_SPEC>;
#[doc = "Wide Arithmetic Configuration"]
pub mod wac;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "DSTATUS register accessor: an alias for `Reg<DSTATUS_SPEC>`"]
pub type DSTATUS = crate::Reg<dstatus::DSTATUS_SPEC>;
#[doc = "Data Status Register"]
pub mod dstatus;
#[doc = "CSTATUS register accessor: an alias for `Reg<CSTATUS_SPEC>`"]
pub type CSTATUS = crate::Reg<cstatus::CSTATUS_SPEC>;
#[doc = "Control Status Register"]
pub mod cstatus;
#[doc = "KEY register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "KEY Register Access"]
pub mod key;
#[doc = "KEYBUF register accessor: an alias for `Reg<KEYBUF_SPEC>`"]
pub type KEYBUF = crate::Reg<keybuf::KEYBUF_SPEC>;
#[doc = "KEY Buffer Register Access"]
pub mod keybuf;
#[doc = "SEQCTRL register accessor: an alias for `Reg<SEQCTRL_SPEC>`"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "SEQCTRLB register accessor: an alias for `Reg<SEQCTRLB_SPEC>`"]
pub type SEQCTRLB = crate::Reg<seqctrlb::SEQCTRLB_SPEC>;
#[doc = "Sequence Control B"]
pub mod seqctrlb;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "AES Interrupt Flags"]
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
#[doc = "SEQ0 register accessor: an alias for `Reg<SEQ0_SPEC>`"]
pub type SEQ0 = crate::Reg<seq0::SEQ0_SPEC>;
#[doc = "Sequence Register 0"]
pub mod seq0;
#[doc = "SEQ1 register accessor: an alias for `Reg<SEQ1_SPEC>`"]
pub type SEQ1 = crate::Reg<seq1::SEQ1_SPEC>;
#[doc = "Sequence Register 1"]
pub mod seq1;
#[doc = "SEQ2 register accessor: an alias for `Reg<SEQ2_SPEC>`"]
pub type SEQ2 = crate::Reg<seq2::SEQ2_SPEC>;
#[doc = "Sequence Register 2"]
pub mod seq2;
#[doc = "SEQ3 register accessor: an alias for `Reg<SEQ3_SPEC>`"]
pub type SEQ3 = crate::Reg<seq3::SEQ3_SPEC>;
#[doc = "Sequence Register 3"]
pub mod seq3;
#[doc = "SEQ4 register accessor: an alias for `Reg<SEQ4_SPEC>`"]
pub type SEQ4 = crate::Reg<seq4::SEQ4_SPEC>;
#[doc = "Sequence Register 4"]
pub mod seq4;
#[doc = "DATA0 register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "DATA0 Register Access"]
pub mod data0;
#[doc = "DATA1 register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "DATA1 Register Access"]
pub mod data1;
#[doc = "DATA2 register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "DATA2 Register Access"]
pub mod data2;
#[doc = "DATA3 register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "DATA3 Register Access"]
pub mod data3;
#[doc = "DATA0XOR register accessor: an alias for `Reg<DATA0XOR_SPEC>`"]
pub type DATA0XOR = crate::Reg<data0xor::DATA0XOR_SPEC>;
#[doc = "DATA0XOR Register Access"]
pub mod data0xor;
#[doc = "DATA0BYTE register accessor: an alias for `Reg<DATA0BYTE_SPEC>`"]
pub type DATA0BYTE = crate::Reg<data0byte::DATA0BYTE_SPEC>;
#[doc = "DATA0 Register Byte Access"]
pub mod data0byte;
#[doc = "DATA1BYTE register accessor: an alias for `Reg<DATA1BYTE_SPEC>`"]
pub type DATA1BYTE = crate::Reg<data1byte::DATA1BYTE_SPEC>;
#[doc = "DATA1 Register Byte Access"]
pub mod data1byte;
#[doc = "DATA0XORBYTE register accessor: an alias for `Reg<DATA0XORBYTE_SPEC>`"]
pub type DATA0XORBYTE = crate::Reg<data0xorbyte::DATA0XORBYTE_SPEC>;
#[doc = "DATA0 Register Byte XOR Access"]
pub mod data0xorbyte;
#[doc = "DATA0BYTE12 register accessor: an alias for `Reg<DATA0BYTE12_SPEC>`"]
pub type DATA0BYTE12 = crate::Reg<data0byte12::DATA0BYTE12_SPEC>;
#[doc = "DATA0 Register Byte 12 Access"]
pub mod data0byte12;
#[doc = "DATA0BYTE13 register accessor: an alias for `Reg<DATA0BYTE13_SPEC>`"]
pub type DATA0BYTE13 = crate::Reg<data0byte13::DATA0BYTE13_SPEC>;
#[doc = "DATA0 Register Byte 13 Access"]
pub mod data0byte13;
#[doc = "DATA0BYTE14 register accessor: an alias for `Reg<DATA0BYTE14_SPEC>`"]
pub type DATA0BYTE14 = crate::Reg<data0byte14::DATA0BYTE14_SPEC>;
#[doc = "DATA0 Register Byte 14 Access"]
pub mod data0byte14;
#[doc = "DATA0BYTE15 register accessor: an alias for `Reg<DATA0BYTE15_SPEC>`"]
pub type DATA0BYTE15 = crate::Reg<data0byte15::DATA0BYTE15_SPEC>;
#[doc = "DATA0 Register Byte 15 Access"]
pub mod data0byte15;
#[doc = "DDATA0 register accessor: an alias for `Reg<DDATA0_SPEC>`"]
pub type DDATA0 = crate::Reg<ddata0::DDATA0_SPEC>;
#[doc = "DDATA0 Register Access"]
pub mod ddata0;
#[doc = "DDATA1 register accessor: an alias for `Reg<DDATA1_SPEC>`"]
pub type DDATA1 = crate::Reg<ddata1::DDATA1_SPEC>;
#[doc = "DDATA1 Register Access"]
pub mod ddata1;
#[doc = "DDATA2 register accessor: an alias for `Reg<DDATA2_SPEC>`"]
pub type DDATA2 = crate::Reg<ddata2::DDATA2_SPEC>;
#[doc = "DDATA2 Register Access"]
pub mod ddata2;
#[doc = "DDATA3 register accessor: an alias for `Reg<DDATA3_SPEC>`"]
pub type DDATA3 = crate::Reg<ddata3::DDATA3_SPEC>;
#[doc = "DDATA3 Register Access"]
pub mod ddata3;
#[doc = "DDATA4 register accessor: an alias for `Reg<DDATA4_SPEC>`"]
pub type DDATA4 = crate::Reg<ddata4::DDATA4_SPEC>;
#[doc = "DDATA4 Register Access"]
pub mod ddata4;
#[doc = "DDATA0BIG register accessor: an alias for `Reg<DDATA0BIG_SPEC>`"]
pub type DDATA0BIG = crate::Reg<ddata0big::DDATA0BIG_SPEC>;
#[doc = "DDATA0 Register Big Endian Access"]
pub mod ddata0big;
#[doc = "DDATA0BYTE register accessor: an alias for `Reg<DDATA0BYTE_SPEC>`"]
pub type DDATA0BYTE = crate::Reg<ddata0byte::DDATA0BYTE_SPEC>;
#[doc = "DDATA0 Register Byte Access"]
pub mod ddata0byte;
#[doc = "DDATA1BYTE register accessor: an alias for `Reg<DDATA1BYTE_SPEC>`"]
pub type DDATA1BYTE = crate::Reg<ddata1byte::DDATA1BYTE_SPEC>;
#[doc = "DDATA1 Register Byte Access"]
pub mod ddata1byte;
#[doc = "DDATA0BYTE32 register accessor: an alias for `Reg<DDATA0BYTE32_SPEC>`"]
pub type DDATA0BYTE32 = crate::Reg<ddata0byte32::DDATA0BYTE32_SPEC>;
#[doc = "DDATA0 Register Byte 32 Access"]
pub mod ddata0byte32;
#[doc = "QDATA0 register accessor: an alias for `Reg<QDATA0_SPEC>`"]
pub type QDATA0 = crate::Reg<qdata0::QDATA0_SPEC>;
#[doc = "QDATA0 Register Access"]
pub mod qdata0;
#[doc = "QDATA1 register accessor: an alias for `Reg<QDATA1_SPEC>`"]
pub type QDATA1 = crate::Reg<qdata1::QDATA1_SPEC>;
#[doc = "QDATA1 Register Access"]
pub mod qdata1;
#[doc = "QDATA1BIG register accessor: an alias for `Reg<QDATA1BIG_SPEC>`"]
pub type QDATA1BIG = crate::Reg<qdata1big::QDATA1BIG_SPEC>;
#[doc = "QDATA1 Register Big Endian Access"]
pub mod qdata1big;
#[doc = "QDATA0BYTE register accessor: an alias for `Reg<QDATA0BYTE_SPEC>`"]
pub type QDATA0BYTE = crate::Reg<qdata0byte::QDATA0BYTE_SPEC>;
#[doc = "QDATA0 Register Byte Access"]
pub mod qdata0byte;
#[doc = "QDATA1BYTE register accessor: an alias for `Reg<QDATA1BYTE_SPEC>`"]
pub type QDATA1BYTE = crate::Reg<qdata1byte::QDATA1BYTE_SPEC>;
#[doc = "QDATA1 Register Byte Access"]
pub mod qdata1byte;
