#[doc = "Register `CSTATUS` reader"]
pub struct R(crate::R<CSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `V0` reader - Selected ALU Operand 0"]
pub type V0_R = crate::FieldReader<u8, V0_A>;
#[doc = "Selected ALU Operand 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V0_A {
    #[doc = "0: `0`"]
    DDATA0 = 0,
    #[doc = "1: `1`"]
    DDATA1 = 1,
    #[doc = "2: `10`"]
    DDATA2 = 2,
    #[doc = "3: `11`"]
    DDATA3 = 3,
    #[doc = "4: `100`"]
    DDATA4 = 4,
    #[doc = "5: `101`"]
    DATA0 = 5,
    #[doc = "6: `110`"]
    DATA1 = 6,
    #[doc = "7: `111`"]
    DATA2 = 7,
}
impl From<V0_A> for u8 {
    #[inline(always)]
    fn from(variant: V0_A) -> Self {
        variant as _
    }
}
impl V0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V0_A {
        match self.bits {
            0 => V0_A::DDATA0,
            1 => V0_A::DDATA1,
            2 => V0_A::DDATA2,
            3 => V0_A::DDATA3,
            4 => V0_A::DDATA4,
            5 => V0_A::DATA0,
            6 => V0_A::DATA1,
            7 => V0_A::DATA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DDATA0`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == V0_A::DDATA0
    }
    #[doc = "Checks if the value of the field is `DDATA1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == V0_A::DDATA1
    }
    #[doc = "Checks if the value of the field is `DDATA2`"]
    #[inline(always)]
    pub fn is_ddata2(&self) -> bool {
        *self == V0_A::DDATA2
    }
    #[doc = "Checks if the value of the field is `DDATA3`"]
    #[inline(always)]
    pub fn is_ddata3(&self) -> bool {
        *self == V0_A::DDATA3
    }
    #[doc = "Checks if the value of the field is `DDATA4`"]
    #[inline(always)]
    pub fn is_ddata4(&self) -> bool {
        *self == V0_A::DDATA4
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == V0_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == V0_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == V0_A::DATA2
    }
}
#[doc = "Field `V1` reader - Selected ALU Operand 1"]
pub type V1_R = crate::FieldReader<u8, V1_A>;
#[doc = "Selected ALU Operand 1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V1_A {
    #[doc = "0: `0`"]
    DDATA0 = 0,
    #[doc = "1: `1`"]
    DDATA1 = 1,
    #[doc = "2: `10`"]
    DDATA2 = 2,
    #[doc = "3: `11`"]
    DDATA3 = 3,
    #[doc = "4: `100`"]
    DDATA4 = 4,
    #[doc = "5: `101`"]
    DATA0 = 5,
    #[doc = "6: `110`"]
    DATA1 = 6,
    #[doc = "7: `111`"]
    DATA2 = 7,
}
impl From<V1_A> for u8 {
    #[inline(always)]
    fn from(variant: V1_A) -> Self {
        variant as _
    }
}
impl V1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V1_A {
        match self.bits {
            0 => V1_A::DDATA0,
            1 => V1_A::DDATA1,
            2 => V1_A::DDATA2,
            3 => V1_A::DDATA3,
            4 => V1_A::DDATA4,
            5 => V1_A::DATA0,
            6 => V1_A::DATA1,
            7 => V1_A::DATA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DDATA0`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == V1_A::DDATA0
    }
    #[doc = "Checks if the value of the field is `DDATA1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == V1_A::DDATA1
    }
    #[doc = "Checks if the value of the field is `DDATA2`"]
    #[inline(always)]
    pub fn is_ddata2(&self) -> bool {
        *self == V1_A::DDATA2
    }
    #[doc = "Checks if the value of the field is `DDATA3`"]
    #[inline(always)]
    pub fn is_ddata3(&self) -> bool {
        *self == V1_A::DDATA3
    }
    #[doc = "Checks if the value of the field is `DDATA4`"]
    #[inline(always)]
    pub fn is_ddata4(&self) -> bool {
        *self == V1_A::DDATA4
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == V1_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == V1_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == V1_A::DATA2
    }
}
#[doc = "Field `SEQPART` reader - Sequence Part"]
pub type SEQPART_R = crate::BitReader<bool>;
#[doc = "Field `SEQSKIP` reader - Sequence Skip Next Instruction"]
pub type SEQSKIP_R = crate::BitReader<bool>;
#[doc = "Field `SEQIP` reader - Sequence Next Instruction Pointer"]
pub type SEQIP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Selected ALU Operand 0"]
    #[inline(always)]
    pub fn v0(&self) -> V0_R {
        V0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selected ALU Operand 1"]
    #[inline(always)]
    pub fn v1(&self) -> V1_R {
        V1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Sequence Part"]
    #[inline(always)]
    pub fn seqpart(&self) -> SEQPART_R {
        SEQPART_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sequence Skip Next Instruction"]
    #[inline(always)]
    pub fn seqskip(&self) -> SEQSKIP_R {
        SEQSKIP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:24 - Sequence Next Instruction Pointer"]
    #[inline(always)]
    pub fn seqip(&self) -> SEQIP_R {
        SEQIP_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
#[doc = "Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cstatus](index.html) module"]
pub struct CSTATUS_SPEC;
impl crate::RegisterSpec for CSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cstatus::R](R) reader structure"]
impl crate::Readable for CSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSTATUS to value 0x0201"]
impl crate::Resettable for CSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0201;
}
