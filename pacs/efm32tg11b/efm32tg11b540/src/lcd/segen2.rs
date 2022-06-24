#[doc = "Register `SEGEN2` reader"]
pub struct R(crate::R<SEGEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGEN2` writer"]
pub struct W(crate::W<SEGEN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGEN2_SPEC>;
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
impl From<crate::W<SEGEN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGEN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGEN2` reader - Segment Enable (second Group)"]
pub type SEGEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGEN2` writer - Segment Enable (second Group)"]
pub type SEGEN2_W<'a> = crate::FieldWriter<'a, u32, SEGEN2_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&self) -> SEGEN2_R {
        SEGEN2_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&mut self) -> SEGEN2_W {
        SEGEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Enable (32 to 39)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segen2](index.html) module"]
pub struct SEGEN2_SPEC;
impl crate::RegisterSpec for SEGEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segen2::R](R) reader structure"]
impl crate::Readable for SEGEN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segen2::W](W) writer structure"]
impl crate::Writable for SEGEN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGEN2 to value 0"]
impl crate::Resettable for SEGEN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
