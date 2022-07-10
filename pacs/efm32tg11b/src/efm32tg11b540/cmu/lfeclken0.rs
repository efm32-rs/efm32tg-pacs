#[doc = "Register `LFECLKEN0` reader"]
pub struct R(crate::R<LFECLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFECLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFECLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFECLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFECLKEN0` writer"]
pub struct W(crate::W<LFECLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFECLKEN0_SPEC>;
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
impl From<crate::W<LFECLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFECLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Clock Enable"]
pub type RTCC_R = crate::BitReader<bool>;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar Clock Enable"]
pub type RTCC_W<'a> = crate::BitWriter<'a, u32, LFECLKEN0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfeclken0](index.html) module"]
pub struct LFECLKEN0_SPEC;
impl crate::RegisterSpec for LFECLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfeclken0::R](R) reader structure"]
impl crate::Readable for LFECLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfeclken0::W](W) writer structure"]
impl crate::Writable for LFECLKEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFECLKEN0 to value 0"]
impl crate::Resettable for LFECLKEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
