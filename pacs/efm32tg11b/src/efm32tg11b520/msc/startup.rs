#[doc = "Register `STARTUP` reader"]
pub struct R(crate::R<STARTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTUP` writer"]
pub struct W(crate::W<STARTUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTUP_SPEC>;
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
impl From<crate::W<STARTUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STDLY0` reader - Startup Delay 0"]
pub type STDLY0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STDLY0` writer - Startup Delay 0"]
pub type STDLY0_W<'a> = crate::FieldWriter<'a, u32, STARTUP_SPEC, u16, u16, 10, 0>;
#[doc = "Field `STDLY1` reader - Startup Delay 0"]
pub type STDLY1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STDLY1` writer - Startup Delay 0"]
pub type STDLY1_W<'a> = crate::FieldWriter<'a, u32, STARTUP_SPEC, u16, u16, 10, 12>;
#[doc = "Field `ASTWAIT` reader - Active Startup Wait"]
pub type ASTWAIT_R = crate::BitReader<bool>;
#[doc = "Field `ASTWAIT` writer - Active Startup Wait"]
pub type ASTWAIT_W<'a> = crate::BitWriter<'a, u32, STARTUP_SPEC, bool, 24>;
#[doc = "Field `STWSEN` reader - Startup Waitstates Enable"]
pub type STWSEN_R = crate::BitReader<bool>;
#[doc = "Field `STWSEN` writer - Startup Waitstates Enable"]
pub type STWSEN_W<'a> = crate::BitWriter<'a, u32, STARTUP_SPEC, bool, 25>;
#[doc = "Field `STWSAEN` reader - Startup Waitstates Always Enable"]
pub type STWSAEN_R = crate::BitReader<bool>;
#[doc = "Field `STWSAEN` writer - Startup Waitstates Always Enable"]
pub type STWSAEN_W<'a> = crate::BitWriter<'a, u32, STARTUP_SPEC, bool, 26>;
#[doc = "Field `STWS` reader - Startup Waitstates"]
pub type STWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STWS` writer - Startup Waitstates"]
pub type STWS_W<'a> = crate::FieldWriter<'a, u32, STARTUP_SPEC, u8, u8, 3, 28>;
impl R {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly0(&self) -> STDLY0_R {
        STDLY0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly1(&self) -> STDLY1_R {
        STDLY1_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    pub fn astwait(&self) -> ASTWAIT_R {
        ASTWAIT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    pub fn stwsen(&self) -> STWSEN_R {
        STWSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    pub fn stwsaen(&self) -> STWSAEN_R {
        STWSAEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    pub fn stws(&self) -> STWS_R {
        STWS_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly0(&mut self) -> STDLY0_W {
        STDLY0_W::new(self)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly1(&mut self) -> STDLY1_W {
        STDLY1_W::new(self)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    pub fn astwait(&mut self) -> ASTWAIT_W {
        ASTWAIT_W::new(self)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    pub fn stwsen(&mut self) -> STWSEN_W {
        STWSEN_W::new(self)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    pub fn stwsaen(&mut self) -> STWSAEN_W {
        STWSAEN_W::new(self)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    pub fn stws(&mut self) -> STWS_W {
        STWS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Startup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startup](index.html) module"]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startup::R](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startup::W](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTUP to value 0x1300_104d"]
impl crate::Resettable for STARTUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1300_104d
    }
}
