#[doc = "Register `HFRCOSS` reader"]
pub struct R(crate::R<HFRCOSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFRCOSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFRCOSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFRCOSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFRCOSS` writer"]
pub struct W(crate::W<HFRCOSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFRCOSS_SPEC>;
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
impl From<crate::W<HFRCOSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFRCOSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSAMP` reader - Spread Spectrum Amplitude"]
pub type SSAMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSAMP` writer - Spread Spectrum Amplitude"]
pub type SSAMP_W<'a> = crate::FieldWriter<'a, u32, HFRCOSS_SPEC, u8, u8, 3, 0>;
#[doc = "Field `SSINV` reader - Spread Spectrum Update Interval"]
pub type SSINV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSINV` writer - Spread Spectrum Update Interval"]
pub type SSINV_W<'a> = crate::FieldWriter<'a, u32, HFRCOSS_SPEC, u8, u8, 5, 8>;
impl R {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&self) -> SSAMP_R {
        SSAMP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&self) -> SSINV_R {
        SSINV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&mut self) -> SSAMP_W {
        SSAMP_W::new(self)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&mut self) -> SSINV_W {
        SSINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFRCO Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrcoss](index.html) module"]
pub struct HFRCOSS_SPEC;
impl crate::RegisterSpec for HFRCOSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfrcoss::R](R) reader structure"]
impl crate::Readable for HFRCOSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfrcoss::W](W) writer structure"]
impl crate::Writable for HFRCOSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFRCOSS to value 0"]
impl crate::Resettable for HFRCOSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
