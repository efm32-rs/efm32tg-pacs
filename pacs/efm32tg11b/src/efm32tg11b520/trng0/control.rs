#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - TRNG Module Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - TRNG Module Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `TESTEN` reader - Test Enable"]
pub type TESTEN_R = crate::BitReader<bool>;
#[doc = "Field `TESTEN` writer - Test Enable"]
pub type TESTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `CONDBYPASS` reader - Conditioning Bypass"]
pub type CONDBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `CONDBYPASS` writer - Conditioning Bypass"]
pub type CONDBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `REPCOUNTIEN` reader - Interrupt Enable for Repetition Count Test Failure"]
pub type REPCOUNTIEN_R = crate::BitReader<bool>;
#[doc = "Field `REPCOUNTIEN` writer - Interrupt Enable for Repetition Count Test Failure"]
pub type REPCOUNTIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `APT64IEN` reader - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
pub type APT64IEN_R = crate::BitReader<bool>;
#[doc = "Field `APT64IEN` writer - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
pub type APT64IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `APT4096IEN` reader - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
pub type APT4096IEN_R = crate::BitReader<bool>;
#[doc = "Field `APT4096IEN` writer - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
pub type APT4096IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `FULLIEN` reader - Interrupt Enable for FIFO Full"]
pub type FULLIEN_R = crate::BitReader<bool>;
#[doc = "Field `FULLIEN` writer - Interrupt Enable for FIFO Full"]
pub type FULLIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `SOFTRESET` reader - Software Reset"]
pub type SOFTRESET_R = crate::BitReader<bool>;
#[doc = "Field `SOFTRESET` writer - Software Reset"]
pub type SOFTRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `PREIEN` reader - Interrupt enable for AIS31 preliminary noise alarm"]
pub type PREIEN_R = crate::BitReader<bool>;
#[doc = "Field `PREIEN` writer - Interrupt enable for AIS31 preliminary noise alarm"]
pub type PREIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALMIEN` reader - Interrupt enable for AIS31 noise alarm"]
pub type ALMIEN_R = crate::BitReader<bool>;
#[doc = "Field `ALMIEN` writer - Interrupt enable for AIS31 noise alarm"]
pub type ALMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `FORCERUN` reader - Oscillator Force Run"]
pub type FORCERUN_R = crate::BitReader<bool>;
#[doc = "Field `FORCERUN` writer - Oscillator Force Run"]
pub type FORCERUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `BYPNIST` reader - NIST Start-up Test Bypass."]
pub type BYPNIST_R = crate::BitReader<bool>;
#[doc = "Field `BYPNIST` writer - NIST Start-up Test Bypass."]
pub type BYPNIST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `BYPAIS31` reader - AIS31 Start-up Test Bypass."]
pub type BYPAIS31_R = crate::BitReader<bool>;
#[doc = "Field `BYPAIS31` writer - AIS31 Start-up Test Bypass."]
pub type BYPAIS31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&self) -> CONDBYPASS_R {
        CONDBYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    pub fn repcountien(&self) -> REPCOUNTIEN_R {
        REPCOUNTIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    pub fn apt64ien(&self) -> APT64IEN_R {
        APT64IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    pub fn apt4096ien(&self) -> APT4096IEN_R {
        APT4096IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    pub fn fullien(&self) -> FULLIEN_R {
        FULLIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    pub fn preien(&self) -> PREIEN_R {
        PREIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&self) -> FORCERUN_R {
        FORCERUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&self) -> BYPNIST_R {
        BYPNIST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&self) -> BYPAIS31_R {
        BYPAIS31_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TESTEN_W<2> {
        TESTEN_W::new(self)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn condbypass(&mut self) -> CONDBYPASS_W<3> {
        CONDBYPASS_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    #[must_use]
    pub fn repcountien(&mut self) -> REPCOUNTIEN_W<4> {
        REPCOUNTIEN_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    #[must_use]
    pub fn apt64ien(&mut self) -> APT64IEN_W<5> {
        APT64IEN_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    #[must_use]
    pub fn apt4096ien(&mut self) -> APT4096IEN_W<6> {
        APT4096IEN_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn fullien(&mut self) -> FULLIEN_W<7> {
        FULLIEN_W::new(self)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SOFTRESET_W<8> {
        SOFTRESET_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    #[must_use]
    pub fn preien(&mut self) -> PREIEN_W<9> {
        PREIEN_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    #[must_use]
    pub fn almien(&mut self) -> ALMIEN_W<10> {
        ALMIEN_W::new(self)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    #[must_use]
    pub fn forcerun(&mut self) -> FORCERUN_W<11> {
        FORCERUN_W::new(self)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypnist(&mut self) -> BYPNIST_W<12> {
        BYPNIST_W::new(self)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypais31(&mut self) -> BYPAIS31_W<13> {
        BYPAIS31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
