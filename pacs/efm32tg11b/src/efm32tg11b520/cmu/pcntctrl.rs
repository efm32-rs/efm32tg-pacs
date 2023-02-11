#[doc = "Register `PCNTCTRL` reader"]
pub struct R(crate::R<PCNTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNTCTRL` writer"]
pub struct W(crate::W<PCNTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNTCTRL_SPEC>;
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
impl From<crate::W<PCNTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCNT0CLKEN` reader - PCNT0 Clock Enable"]
pub type PCNT0CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0CLKEN` writer - PCNT0 Clock Enable"]
pub type PCNT0CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
#[doc = "Field `PCNT0CLKSEL` reader - PCNT0 Clock Select"]
pub type PCNT0CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0CLKSEL` writer - PCNT0 Clock Select"]
pub type PCNT0CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> PCNT0CLKEN_R {
        PCNT0CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> PCNT0CLKSEL_R {
        PCNT0CLKSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clken(&mut self) -> PCNT0CLKEN_W<0> {
        PCNT0CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clksel(&mut self) -> PCNT0CLKSEL_W<1> {
        PCNT0CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCNT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntctrl](index.html) module"]
pub struct PCNTCTRL_SPEC;
impl crate::RegisterSpec for PCNTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcntctrl::R](R) reader structure"]
impl crate::Readable for PCNTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcntctrl::W](W) writer structure"]
impl crate::Writable for PCNTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNTCTRL to value 0"]
impl crate::Resettable for PCNTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
