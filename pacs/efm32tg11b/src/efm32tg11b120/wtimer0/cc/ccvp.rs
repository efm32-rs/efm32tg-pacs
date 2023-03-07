#[doc = "Register `CCVP` reader"]
pub struct R(crate::R<CCVP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCVP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCVP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCVP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CCVP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new(self.bits)
    }
}
#[doc = "CC Channel Value Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccvp](index.html) module"]
pub struct CCVP_SPEC;
impl crate::RegisterSpec for CCVP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccvp::R](R) reader structure"]
impl crate::Readable for CCVP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCVP to value 0"]
impl crate::Resettable for CCVP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
