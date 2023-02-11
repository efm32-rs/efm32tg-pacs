#[doc = "Register `SINGLEFIFOCOUNT` reader"]
pub struct R(crate::R<SINGLEFIFOCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLEFIFOCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLEFIFOCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLEFIFOCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SINGLEDC` reader - Single Data Count"]
pub type SINGLEDC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Single Data Count"]
    #[inline(always)]
    pub fn singledc(&self) -> SINGLEDC_R {
        SINGLEDC_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Single FIFO Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlefifocount](index.html) module"]
pub struct SINGLEFIFOCOUNT_SPEC;
impl crate::RegisterSpec for SINGLEFIFOCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlefifocount::R](R) reader structure"]
impl crate::Readable for SINGLEFIFOCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SINGLEFIFOCOUNT to value 0"]
impl crate::Resettable for SINGLEFIFOCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
