#[doc = "Register `HFXOSTEADYSTATECTRL` reader"]
pub struct R(crate::R<HFXOSTEADYSTATECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSTEADYSTATECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSTEADYSTATECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSTEADYSTATECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOSTEADYSTATECTRL` writer"]
pub struct W(crate::W<HFXOSTEADYSTATECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSTEADYSTATECTRL_SPEC>;
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
impl From<crate::W<HFXOSTEADYSTATECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSTEADYSTATECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_W<'a> =
    crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u16, u16, 11, 0>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a> = crate::FieldWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, u16, u16, 9, 11>;
#[doc = "Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_R = crate::BitReader<bool>;
#[doc = "Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_W<'a> = crate::BitWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, bool, 26>;
#[doc = "Field `PEAKMONEN` reader - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
pub type PEAKMONEN_R = crate::BitReader<bool>;
#[doc = "Field `PEAKMONEN` writer - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
pub type PEAKMONEN_W<'a> = crate::BitWriter<'a, u32, HFXOSTEADYSTATECTRL_SPEC, bool, 27>;
impl R {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PEAKDETEN_R {
        PEAKDETEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&self) -> PEAKMONEN_R {
        PEAKMONEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W {
        IBTRIMXOCORE_W::new(self)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CTUNE_W {
        CTUNE_W::new(self)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W {
        PEAKDETEN_W::new(self)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&mut self) -> PEAKMONEN_W {
        PEAKMONEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Steady State Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosteadystatectrl](index.html) module"]
pub struct HFXOSTEADYSTATECTRL_SPEC;
impl crate::RegisterSpec for HFXOSTEADYSTATECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxosteadystatectrl::R](R) reader structure"]
impl crate::Readable for HFXOSTEADYSTATECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxosteadystatectrl::W](W) writer structure"]
impl crate::Writable for HFXOSTEADYSTATECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOSTEADYSTATECTRL to value 0x0800_0100"]
impl crate::Resettable for HFXOSTEADYSTATECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800_0100
    }
}
