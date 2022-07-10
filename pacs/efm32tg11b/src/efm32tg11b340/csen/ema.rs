#[doc = "Register `EMA` reader"]
pub struct R(crate::R<EMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMA` writer"]
pub struct W(crate::W<EMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMA_SPEC>;
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
impl From<crate::W<EMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMA` reader - Calculated Exponential Moving Average"]
pub type EMA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMA` writer - Calculated Exponential Moving Average"]
pub type EMA_W<'a> = crate::FieldWriter<'a, u32, EMA_SPEC, u32, u32, 22, 0>;
impl R {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&self) -> EMA_R {
        EMA_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&mut self) -> EMA_W {
        EMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Exponential Moving Average\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ema](index.html) module"]
pub struct EMA_SPEC;
impl crate::RegisterSpec for EMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ema::R](R) reader structure"]
impl crate::Readable for EMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ema::W](W) writer structure"]
impl crate::Writable for EMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMA to value 0"]
impl crate::Resettable for EMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
