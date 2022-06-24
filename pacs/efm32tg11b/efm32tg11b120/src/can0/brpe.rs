#[doc = "Register `BRPE` reader"]
pub struct R(crate::R<BRPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRPE` writer"]
pub struct W(crate::W<BRPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRPE_SPEC>;
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
impl From<crate::W<BRPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRPE` reader - Baud Rate Prescaler Extension"]
pub type BRPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRPE` writer - Baud Rate Prescaler Extension"]
pub type BRPE_W<'a> = crate::FieldWriter<'a, u32, BRPE_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Baud Rate Prescaler Extension"]
    #[inline(always)]
    pub fn brpe(&self) -> BRPE_R {
        BRPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Prescaler Extension"]
    #[inline(always)]
    pub fn brpe(&mut self) -> BRPE_W {
        BRPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BRP Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brpe](index.html) module"]
pub struct BRPE_SPEC;
impl crate::RegisterSpec for BRPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brpe::R](R) reader structure"]
impl crate::Readable for BRPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brpe::W](W) writer structure"]
impl crate::Writable for BRPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRPE to value 0"]
impl crate::Resettable for BRPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
