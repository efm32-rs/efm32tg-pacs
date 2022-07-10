#[doc = "Register `LINKLOAD` writer"]
pub struct W(crate::W<LINKLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINKLOAD_SPEC>;
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
impl From<crate::W<LINKLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINKLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINKLOAD` writer - DMA Link Loads"]
pub type LINKLOAD_W<'a> = crate::FieldWriter<'a, u32, LINKLOAD_SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bits 0:7 - DMA Link Loads"]
    #[inline(always)]
    pub fn linkload(&mut self) -> LINKLOAD_W {
        LINKLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Link Load Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linkload](index.html) module"]
pub struct LINKLOAD_SPEC;
impl crate::RegisterSpec for LINKLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [linkload::W](W) writer structure"]
impl crate::Writable for LINKLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINKLOAD to value 0"]
impl crate::Resettable for LINKLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
