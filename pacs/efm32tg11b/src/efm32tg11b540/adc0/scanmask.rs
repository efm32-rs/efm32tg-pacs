#[doc = "Register `SCANMASK` reader"]
pub struct R(crate::R<SCANMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANMASK` writer"]
pub struct W(crate::W<SCANMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANMASK_SPEC>;
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
impl From<crate::W<SCANMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANINPUTEN` reader - Scan Sequence Input Mask"]
pub type SCANINPUTEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Sequence Input Mask"]
pub type SCANINPUTEN_W<'a> = crate::FieldWriter<'a, u32, SCANMASK_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Scan Sequence Input Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Sequence Input Mask"]
    #[inline(always)]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W {
        SCANINPUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Sequence Input Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanmask](index.html) module"]
pub struct SCANMASK_SPEC;
impl crate::RegisterSpec for SCANMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanmask::R](R) reader structure"]
impl crate::Readable for SCANMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanmask::W](W) writer structure"]
impl crate::Writable for SCANMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCANMASK to value 0"]
impl crate::Resettable for SCANMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
