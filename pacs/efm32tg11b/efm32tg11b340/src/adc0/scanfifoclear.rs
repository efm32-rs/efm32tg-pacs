#[doc = "Register `SCANFIFOCLEAR` writer"]
pub struct W(crate::W<SCANFIFOCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANFIFOCLEAR_SPEC>;
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
impl From<crate::W<SCANFIFOCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANFIFOCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANFIFOCLEAR` writer - Clear Scan FIFO Content"]
pub type SCANFIFOCLEAR_W<'a> = crate::BitWriter<'a, u32, SCANFIFOCLEAR_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - Clear Scan FIFO Content"]
    #[inline(always)]
    pub fn scanfifoclear(&mut self) -> SCANFIFOCLEAR_W {
        SCANFIFOCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan FIFO Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifoclear](index.html) module"]
pub struct SCANFIFOCLEAR_SPEC;
impl crate::RegisterSpec for SCANFIFOCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scanfifoclear::W](W) writer structure"]
impl crate::Writable for SCANFIFOCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCANFIFOCLEAR to value 0"]
impl crate::Resettable for SCANFIFOCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
