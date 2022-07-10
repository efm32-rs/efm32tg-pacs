#[doc = "Register `CACHECMD` writer"]
pub struct W(crate::W<CACHECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHECMD_SPEC>;
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
impl From<crate::W<CACHECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVCACHE` writer - Invalidate Instruction Cache"]
pub type INVCACHE_W<'a> = crate::BitWriter<'a, u32, CACHECMD_SPEC, bool, 0>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type STARTPC_W<'a> = crate::BitWriter<'a, u32, CACHECMD_SPEC, bool, 1>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type STOPPC_W<'a> = crate::BitWriter<'a, u32, CACHECMD_SPEC, bool, 2>;
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    pub fn invcache(&mut self) -> INVCACHE_W {
        INVCACHE_W::new(self)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    pub fn startpc(&mut self) -> STARTPC_W {
        STARTPC_W::new(self)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    pub fn stoppc(&mut self) -> STOPPC_W {
        STOPPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Cache Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachecmd](index.html) module"]
pub struct CACHECMD_SPEC;
impl crate::RegisterSpec for CACHECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cachecmd::W](W) writer structure"]
impl crate::Writable for CACHECMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHECMD to value 0"]
impl crate::Resettable for CACHECMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
