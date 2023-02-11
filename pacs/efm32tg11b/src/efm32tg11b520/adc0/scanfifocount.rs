#[doc = "Register `SCANFIFOCOUNT` reader"]
pub struct R(crate::R<SCANFIFOCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANFIFOCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANFIFOCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANFIFOCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCANDC` reader - Scan Data Count"]
pub type SCANDC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Scan Data Count"]
    #[inline(always)]
    pub fn scandc(&self) -> SCANDC_R {
        SCANDC_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Scan FIFO Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifocount](index.html) module"]
pub struct SCANFIFOCOUNT_SPEC;
impl crate::RegisterSpec for SCANFIFOCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanfifocount::R](R) reader structure"]
impl crate::Readable for SCANFIFOCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANFIFOCOUNT to value 0"]
impl crate::Resettable for SCANFIFOCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
