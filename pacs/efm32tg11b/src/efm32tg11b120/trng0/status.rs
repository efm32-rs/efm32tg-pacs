#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TESTDATABUSY` reader - Test Data Busy"]
pub type TESTDATABUSY_R = crate::BitReader<bool>;
#[doc = "Field `REPCOUNTIF` reader - Repetition Count Test Interrupt Status"]
pub type REPCOUNTIF_R = crate::BitReader<bool>;
#[doc = "Field `APT64IF` reader - Adaptive Proportion test failure (64-sample window) interrupt status"]
pub type APT64IF_R = crate::BitReader<bool>;
#[doc = "Field `APT4096IF` reader - Adaptive Proportion test failure (4096-sample window) interrupt status"]
pub type APT4096IF_R = crate::BitReader<bool>;
#[doc = "Field `FULLIF` reader - FIFO Full Interrupt Status"]
pub type FULLIF_R = crate::BitReader<bool>;
#[doc = "Field `PREIF` reader - AIS31 Preliminary Noise Alarm interrupt status"]
pub type PREIF_R = crate::BitReader<bool>;
#[doc = "Field `PREIF` writer - AIS31 Preliminary Noise Alarm interrupt status"]
pub type PREIF_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 8>;
#[doc = "Field `ALMIF` reader - AIS31 Noise Alarm interrupt status"]
pub type ALMIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Test Data Busy"]
    #[inline(always)]
    pub fn testdatabusy(&self) -> TESTDATABUSY_R {
        TESTDATABUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition Count Test Interrupt Status"]
    #[inline(always)]
    pub fn repcountif(&self) -> REPCOUNTIF_R {
        REPCOUNTIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Adaptive Proportion test failure (64-sample window) interrupt status"]
    #[inline(always)]
    pub fn apt64if(&self) -> APT64IF_R {
        APT64IF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Adaptive Proportion test failure (4096-sample window) interrupt status"]
    #[inline(always)]
    pub fn apt4096if(&self) -> APT4096IF_R {
        APT4096IF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn fullif(&self) -> FULLIF_R {
        FULLIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn preif(&self) -> PREIF_R {
        PREIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AIS31 Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn almif(&self) -> ALMIF_R {
        ALMIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn preif(&mut self) -> PREIF_W {
        PREIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
