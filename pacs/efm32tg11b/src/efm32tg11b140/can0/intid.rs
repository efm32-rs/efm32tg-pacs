#[doc = "Register `INTID` reader"]
pub struct R(crate::R<INTID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTID` reader - Interrupt Identifier"]
pub type INTID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTSTAT` reader - Status Interupt"]
pub type INTSTAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Interrupt Identifier"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Status Interupt"]
    #[inline(always)]
    pub fn intstat(&self) -> INTSTAT_R {
        INTSTAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intid](index.html) module"]
pub struct INTID_SPEC;
impl crate::RegisterSpec for INTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intid::R](R) reader structure"]
impl crate::Readable for INTID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTID to value 0"]
impl crate::Resettable for INTID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
