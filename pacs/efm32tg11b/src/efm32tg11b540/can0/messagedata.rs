#[doc = "Register `MESSAGEDATA` reader"]
pub struct R(crate::R<MESSAGEDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MESSAGEDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MESSAGEDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MESSAGEDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID` reader - DATAVALID Bits (of All Message Objects)"]
pub type VALID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATAVALID Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(self.bits)
    }
}
#[doc = "New Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [messagedata](index.html) module"]
pub struct MESSAGEDATA_SPEC;
impl crate::RegisterSpec for MESSAGEDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [messagedata::R](R) reader structure"]
impl crate::Readable for MESSAGEDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MESSAGEDATA to value 0"]
impl crate::Resettable for MESSAGEDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
