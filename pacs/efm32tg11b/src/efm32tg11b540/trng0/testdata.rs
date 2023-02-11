#[doc = "Register `TESTDATA` reader"]
pub struct R(crate::R<TESTDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESTDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TESTDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TESTDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TESTDATA` writer"]
pub struct W(crate::W<TESTDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESTDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TESTDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TESTDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Test data input to conditioning function or to the continuous tests"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Test data input to conditioning function or to the continuous tests"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TESTDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Test data input to conditioning function or to the continuous tests"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Test data input to conditioning function or to the continuous tests"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testdata](index.html) module"]
pub struct TESTDATA_SPEC;
impl crate::RegisterSpec for TESTDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [testdata::R](R) reader structure"]
impl crate::Readable for TESTDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [testdata::W](W) writer structure"]
impl crate::Writable for TESTDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TESTDATA to value 0"]
impl crate::Resettable for TESTDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
