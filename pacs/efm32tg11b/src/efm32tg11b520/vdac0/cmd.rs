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
pub type CH0EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `CH0DIS` writer - DAC Channel 0 Disable"]
pub type CH0DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `CH1EN` writer - DAC Channel 1 Enable"]
pub type CH1EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 2>;
#[doc = "Field `CH1DIS` writer - DAC Channel 1 Disable"]
pub type CH1DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
#[doc = "Field `OPA0EN` writer - OPA0 Enable"]
pub type OPA0EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 16>;
#[doc = "Field `OPA0DIS` writer - OPA0 Disable"]
pub type OPA0DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 17>;
#[doc = "Field `OPA1EN` writer - OPA1 Enable"]
pub type OPA1EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 18>;
#[doc = "Field `OPA1DIS` writer - OPA1 Disable"]
pub type OPA1DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 19>;
#[doc = "Field `OPA2EN` writer - OPA2 Enable"]
pub type OPA2EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 20>;
#[doc = "Field `OPA2DIS` writer - OPA2 Disable"]
pub type OPA2DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 21>;
#[doc = "Field `OPA3EN` writer - OPA3 Enable"]
pub type OPA3EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 22>;
#[doc = "Field `OPA3DIS` writer - OPA3 Disable"]
pub type OPA3DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 23>;
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0dis(&mut self) -> CH0DIS_W {
        CH0DIS_W::new(self)
    }
    #[doc = "Bit 2 - DAC Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 3 - DAC Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1dis(&mut self) -> CH1DIS_W {
        CH1DIS_W::new(self)
    }
    #[doc = "Bit 16 - OPA0 Enable"]
    #[inline(always)]
    pub fn opa0en(&mut self) -> OPA0EN_W {
        OPA0EN_W::new(self)
    }
    #[doc = "Bit 17 - OPA0 Disable"]
    #[inline(always)]
    pub fn opa0dis(&mut self) -> OPA0DIS_W {
        OPA0DIS_W::new(self)
    }
    #[doc = "Bit 18 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1en(&mut self) -> OPA1EN_W {
        OPA1EN_W::new(self)
    }
    #[doc = "Bit 19 - OPA1 Disable"]
    #[inline(always)]
    pub fn opa1dis(&mut self) -> OPA1DIS_W {
        OPA1DIS_W::new(self)
    }
    #[doc = "Bit 20 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2en(&mut self) -> OPA2EN_W {
        OPA2EN_W::new(self)
    }
    #[doc = "Bit 21 - OPA2 Disable"]
    #[inline(always)]
    pub fn opa2dis(&mut self) -> OPA2DIS_W {
        OPA2DIS_W::new(self)
    }
    #[doc = "Bit 22 - OPA3 Enable"]
    #[inline(always)]
    pub fn opa3en(&mut self) -> OPA3EN_W {
        OPA3EN_W::new(self)
    }
    #[doc = "Bit 23 - OPA3 Disable"]
    #[inline(always)]
    pub fn opa3dis(&mut self) -> OPA3DIS_W {
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
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
