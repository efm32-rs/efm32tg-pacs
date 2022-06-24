#[doc = "Register `CC3_CCV` reader"]
pub struct R(crate::R<CC3_CCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC3_CCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC3_CCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC3_CCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC3_CCV` writer"]
pub struct W(crate::W<CC3_CCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC3_CCV_SPEC>;
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
impl From<crate::W<CC3_CCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC3_CCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CCV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CCV_W<'a> = crate::FieldWriter<'a, u32, CC3_CCV_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&mut self) -> CCV_W {
        CCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Channel Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc3_ccv](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct CC3_CCV_SPEC;
impl crate::RegisterSpec for CC3_CCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc3_ccv::R](R) reader structure"]
impl crate::Readable for CC3_CCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc3_ccv::W](W) writer structure"]
impl crate::Writable for CC3_CCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC3_CCV to value 0"]
impl crate::Resettable for CC3_CCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
