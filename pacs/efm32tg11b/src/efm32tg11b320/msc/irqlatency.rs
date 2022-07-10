#[doc = "Register `IRQLATENCY` reader"]
pub struct R(crate::R<IRQLATENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQLATENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQLATENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQLATENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQLATENCY` writer"]
pub struct W(crate::W<IRQLATENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQLATENCY_SPEC>;
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
impl From<crate::W<IRQLATENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQLATENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQLATENCY` reader - Irq Latency Register"]
pub type IRQLATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQLATENCY` writer - Irq Latency Register"]
pub type IRQLATENCY_W<'a> = crate::FieldWriter<'a, u32, IRQLATENCY_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&self) -> IRQLATENCY_R {
        IRQLATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&mut self) -> IRQLATENCY_W {
        IRQLATENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq Latency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqlatency](index.html) module"]
pub struct IRQLATENCY_SPEC;
impl crate::RegisterSpec for IRQLATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqlatency::R](R) reader structure"]
impl crate::Readable for IRQLATENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqlatency::W](W) writer structure"]
impl crate::Writable for IRQLATENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQLATENCY to value 0"]
impl crate::Resettable for IRQLATENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
