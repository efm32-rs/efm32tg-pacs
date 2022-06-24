#[doc = "Register `TEMP` reader"]
pub struct R(crate::R<TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEMP` reader - Temperature Measurement"]
pub type TEMP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Temperature Measurement"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Value of Last Temperature Measurement\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](index.html) module"]
pub struct TEMP_SPEC;
impl crate::RegisterSpec for TEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp::R](R) reader structure"]
impl crate::Readable for TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
