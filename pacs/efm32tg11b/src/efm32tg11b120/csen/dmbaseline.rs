#[doc = "Register `DMBASELINE` reader"]
pub struct R(crate::R<DMBASELINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMBASELINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMBASELINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMBASELINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMBASELINE` writer"]
pub struct W(crate::W<DMBASELINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMBASELINE_SPEC>;
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
impl From<crate::W<DMBASELINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMBASELINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASELINEUP` reader - Delta Modulator Integrator Initial Value"]
pub type BASELINEUP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASELINEUP` writer - Delta Modulator Integrator Initial Value"]
pub type BASELINEUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMBASELINE_SPEC, u16, u16, 16, O>;
#[doc = "Field `BASELINEDN` reader - Delta Modulator Integrator Initial Value"]
pub type BASELINEDN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASELINEDN` writer - Delta Modulator Integrator Initial Value"]
pub type BASELINEDN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMBASELINE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselineup(&self) -> BASELINEUP_R {
        BASELINEUP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselinedn(&self) -> BASELINEDN_R {
        BASELINEDN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn baselineup(&mut self) -> BASELINEUP_W<0> {
        BASELINEUP_W::new(self)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn baselinedn(&mut self) -> BASELINEDN_W<16> {
        BASELINEDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delta Modulation Baseline\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmbaseline](index.html) module"]
pub struct DMBASELINE_SPEC;
impl crate::RegisterSpec for DMBASELINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmbaseline::R](R) reader structure"]
impl crate::Readable for DMBASELINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmbaseline::W](W) writer structure"]
impl crate::Writable for DMBASELINE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMBASELINE to value 0"]
impl crate::Resettable for DMBASELINE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
