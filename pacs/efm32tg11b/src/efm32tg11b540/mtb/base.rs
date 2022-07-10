#[doc = "Register `BASE` reader"]
pub struct R(crate::R<BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE` writer"]
pub struct W(crate::W<BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE_SPEC>;
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
impl From<crate::W<BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - The ram base address."]
pub type BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE` writer - The ram base address."]
pub type BASE_W<'a> = crate::FieldWriter<'a, u32, BASE_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Trace Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](index.html) module"]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base::R](R) reader structure"]
impl crate::Readable for BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base::W](W) writer structure"]
impl crate::Writable for BASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BASE to value 0x2000_0000"]
impl crate::Resettable for BASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
