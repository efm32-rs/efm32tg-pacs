#[doc = "Register `SCANMASK0` reader"]
pub struct R(crate::R<SCANMASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANMASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANMASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANMASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANMASK0` writer"]
pub struct W(crate::W<SCANMASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANMASK0_SPEC>;
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
impl From<crate::W<SCANMASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANMASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANINPUTEN` reader - Scan Channel Mask"]
pub type SCANINPUTEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Channel Mask"]
pub type SCANINPUTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCANMASK0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    #[must_use]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W<0> {
        SCANINPUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Channel Mask 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanmask0](index.html) module"]
pub struct SCANMASK0_SPEC;
impl crate::RegisterSpec for SCANMASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanmask0::R](R) reader structure"]
impl crate::Readable for SCANMASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanmask0::W](W) writer structure"]
impl crate::Writable for SCANMASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANMASK0 to value 0"]
impl crate::Resettable for SCANMASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
