#[doc = "Register `CMPTHR` reader"]
pub struct R(crate::R<CMPTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPTHR` writer"]
pub struct W(crate::W<CMPTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPTHR_SPEC>;
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
impl From<crate::W<CMPTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPTHR` reader - Comparator Threshold."]
pub type CMPTHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPTHR` writer - Comparator Threshold."]
pub type CMPTHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPTHR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    pub fn cmpthr(&self) -> CMPTHR_R {
        CMPTHR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    #[must_use]
    pub fn cmpthr(&mut self) -> CMPTHR_W<0> {
        CMPTHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpthr](index.html) module"]
pub struct CMPTHR_SPEC;
impl crate::RegisterSpec for CMPTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpthr::R](R) reader structure"]
impl crate::Readable for CMPTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpthr::W](W) writer structure"]
impl crate::Writable for CMPTHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPTHR to value 0"]
impl crate::Resettable for CMPTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
