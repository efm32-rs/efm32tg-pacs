#[doc = "Register `SINGLEFIFOCLEAR` writer"]
pub struct W(crate::W<SINGLEFIFOCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLEFIFOCLEAR_SPEC>;
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
impl From<crate::W<SINGLEFIFOCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLEFIFOCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLEFIFOCLEAR` writer - Clear Single FIFO Content"]
pub type SINGLEFIFOCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SINGLEFIFOCLEAR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear Single FIFO Content"]
    #[inline(always)]
    #[must_use]
    pub fn singlefifoclear(&mut self) -> SINGLEFIFOCLEAR_W<0> {
        SINGLEFIFOCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single FIFO Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlefifoclear](index.html) module"]
pub struct SINGLEFIFOCLEAR_SPEC;
impl crate::RegisterSpec for SINGLEFIFOCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [singlefifoclear::W](W) writer structure"]
impl crate::Writable for SINGLEFIFOCLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINGLEFIFOCLEAR to value 0"]
impl crate::Resettable for SINGLEFIFOCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
