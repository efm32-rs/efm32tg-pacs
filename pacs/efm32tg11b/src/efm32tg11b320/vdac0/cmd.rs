#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0EN` writer - DAC Channel 0 Enable"]
pub type CH0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CH0DIS` writer - DAC Channel 0 Disable"]
pub type CH0DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CH1EN` writer - DAC Channel 1 Enable"]
pub type CH1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CH1DIS` writer - DAC Channel 1 Disable"]
pub type CH1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA0EN` writer - OPA0 Enable"]
pub type OPA0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA0DIS` writer - OPA0 Disable"]
pub type OPA0DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA1EN` writer - OPA1 Enable"]
pub type OPA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA1DIS` writer - OPA1 Disable"]
pub type OPA1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA2EN` writer - OPA2 Enable"]
pub type OPA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA2DIS` writer - OPA2 Disable"]
pub type OPA2DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA3EN` writer - OPA3 Enable"]
pub type OPA3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `OPA3DIS` writer - OPA3 Disable"]
pub type OPA3DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> CH0EN_W<0> {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0dis(&mut self) -> CH0DIS_W<1> {
        CH0DIS_W::new(self)
    }
    #[doc = "Bit 2 - DAC Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> CH1EN_W<2> {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 3 - DAC Channel 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1dis(&mut self) -> CH1DIS_W<3> {
        CH1DIS_W::new(self)
    }
    #[doc = "Bit 16 - OPA0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0en(&mut self) -> OPA0EN_W<16> {
        OPA0EN_W::new(self)
    }
    #[doc = "Bit 17 - OPA0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0dis(&mut self) -> OPA0DIS_W<17> {
        OPA0DIS_W::new(self)
    }
    #[doc = "Bit 18 - OPA1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1en(&mut self) -> OPA1EN_W<18> {
        OPA1EN_W::new(self)
    }
    #[doc = "Bit 19 - OPA1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1dis(&mut self) -> OPA1DIS_W<19> {
        OPA1DIS_W::new(self)
    }
    #[doc = "Bit 20 - OPA2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2en(&mut self) -> OPA2EN_W<20> {
        OPA2EN_W::new(self)
    }
    #[doc = "Bit 21 - OPA2 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2dis(&mut self) -> OPA2DIS_W<21> {
        OPA2DIS_W::new(self)
    }
    #[doc = "Bit 22 - OPA3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3en(&mut self) -> OPA3EN_W<22> {
        OPA3EN_W::new(self)
    }
    #[doc = "Bit 23 - OPA3 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3dis(&mut self) -> OPA3DIS_W<23> {
        OPA3DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
