#[doc = "Register `OPA1_TIMER` reader"]
pub struct R(crate::R<OPA1_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA1_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA1_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA1_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA1_TIMER` writer"]
pub struct W(crate::W<OPA1_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA1_TIMER_SPEC>;
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
impl From<crate::W<OPA1_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA1_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTUPDLY` reader - OPAx Startup Delay Count Value"]
pub type STARTUPDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUPDLY` writer - OPAx Startup Delay Count Value"]
pub type STARTUPDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA1_TIMER_SPEC, u8, u8, 6, O>;
#[doc = "Field `WARMUPTIME` reader - OPAx Warmup Time Count Value"]
pub type WARMUPTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WARMUPTIME` writer - OPAx Warmup Time Count Value"]
pub type WARMUPTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA1_TIMER_SPEC, u8, u8, 7, O>;
#[doc = "Field `SETTLETIME` reader - OPAx Output Settling Timeout Value"]
pub type SETTLETIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SETTLETIME` writer - OPAx Output Settling Timeout Value"]
pub type SETTLETIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPA1_TIMER_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    pub fn startupdly(&self) -> STARTUPDLY_R {
        STARTUPDLY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WARMUPTIME_R {
        WARMUPTIME_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    pub fn settletime(&self) -> SETTLETIME_R {
        SETTLETIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn startupdly(&mut self) -> STARTUPDLY_W<0> {
        STARTUPDLY_W::new(self)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn warmuptime(&mut self) -> WARMUPTIME_W<8> {
        WARMUPTIME_W::new(self)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn settletime(&mut self) -> SETTLETIME_W<16> {
        SETTLETIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_timer](index.html) module"]
pub struct OPA1_TIMER_SPEC;
impl crate::RegisterSpec for OPA1_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa1_timer::R](R) reader structure"]
impl crate::Readable for OPA1_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa1_timer::W](W) writer structure"]
impl crate::Writable for OPA1_TIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA1_TIMER to value 0x0001_0700"]
impl crate::Resettable for OPA1_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0700;
}
