#[doc = "Register `AAPUNLOCKCMD` writer"]
pub struct W(crate::W<AAPUNLOCKCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AAPUNLOCKCMD_SPEC>;
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
impl From<crate::W<AAPUNLOCKCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AAPUNLOCKCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCKAAP` writer - Software Unlock AAP Command"]
pub type UNLOCKAAP_W<'a> = crate::BitWriter<'a, u32, AAPUNLOCKCMD_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - Software Unlock AAP Command"]
    #[inline(always)]
    pub fn unlockaap(&mut self) -> UNLOCKAAP_W {
        UNLOCKAAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Unlock AAP Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aapunlockcmd](index.html) module"]
pub struct AAPUNLOCKCMD_SPEC;
impl crate::RegisterSpec for AAPUNLOCKCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aapunlockcmd::W](W) writer structure"]
impl crate::Writable for AAPUNLOCKCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AAPUNLOCKCMD to value 0"]
impl crate::Resettable for AAPUNLOCKCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
