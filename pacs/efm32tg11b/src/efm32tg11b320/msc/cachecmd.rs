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
pub type INVCACHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECMD_SPEC, bool, O>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type STARTPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECMD_SPEC, bool, O>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type STOPPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHECMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    #[must_use]
    pub fn invcache(&mut self) -> INVCACHE_W<0> {
        INVCACHE_W::new(self)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn startpc(&mut self) -> STARTPC_W<1> {
        STARTPC_W::new(self)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn stoppc(&mut self) -> STOPPC_W<2> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHECMD to value 0"]
impl crate::Resettable for CACHECMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
