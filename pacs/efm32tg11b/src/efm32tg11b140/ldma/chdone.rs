#[doc = "Register `CHDONE` reader"]
pub struct R(crate::R<CHDONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHDONE` writer"]
pub struct W(crate::W<CHDONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDONE_SPEC>;
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
impl From<crate::W<CHDONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHDONE` reader - DMA Channel Linking or Done"]
pub type CHDONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHDONE` writer - DMA Channel Linking or Done"]
pub type CHDONE_W<'a> = crate::FieldWriter<'a, u32, CHDONE_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&self) -> CHDONE_R {
        CHDONE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&mut self) -> CHDONE_W {
        CHDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdone](index.html) module"]
pub struct CHDONE_SPEC;
impl crate::RegisterSpec for CHDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdone::R](R) reader structure"]
impl crate::Readable for CHDONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chdone::W](W) writer structure"]
impl crate::Writable for CHDONE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for CHDONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
