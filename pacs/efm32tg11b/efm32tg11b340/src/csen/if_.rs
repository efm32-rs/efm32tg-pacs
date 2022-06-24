#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMP` reader - Digital Comparator Interrupt Flag"]
pub type CMP_R = crate::BitReader<bool>;
#[doc = "Field `CONV` reader - Conversion Done Interrupt Flag"]
pub type CONV_R = crate::BitReader<bool>;
#[doc = "Field `EOS` reader - End of Scan Interrupt Flag."]
pub type EOS_R = crate::BitReader<bool>;
#[doc = "Field `DMAOF` reader - DMA Overflow Interrupt Flag."]
pub type DMAOF_R = crate::BitReader<bool>;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Interrupt Flag"]
pub type APORTCONFLICT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Digital Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Scan Interrupt Flag."]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Overflow Interrupt Flag."]
    #[inline(always)]
    pub fn dmaof(&self) -> DMAOF_R {
        DMAOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
