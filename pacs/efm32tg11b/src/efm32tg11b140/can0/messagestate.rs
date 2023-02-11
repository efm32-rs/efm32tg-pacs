#[doc = "Register `MESSAGESTATE` reader"]
pub struct R(crate::R<MESSAGESTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MESSAGESTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MESSAGESTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MESSAGESTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID` reader - Message Valid Bits (of All Message Objects)"]
pub type VALID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Valid Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(self.bits)
    }
}
#[doc = "Message Valid Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [messagestate](index.html) module"]
pub struct MESSAGESTATE_SPEC;
impl crate::RegisterSpec for MESSAGESTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [messagestate::R](R) reader structure"]
impl crate::Readable for MESSAGESTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MESSAGESTATE to value 0"]
impl crate::Resettable for MESSAGESTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
