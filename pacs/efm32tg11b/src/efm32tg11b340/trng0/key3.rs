#[doc = "Register `KEY3` reader"]
pub struct R(crate::R<KEY3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY3` writer"]
pub struct W(crate::W<KEY3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY3_SPEC>;
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
impl From<crate::W<KEY3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Key 3"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Key 3"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key3](index.html) module"]
pub struct KEY3_SPEC;
impl crate::RegisterSpec for KEY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key3::R](R) reader structure"]
impl crate::Readable for KEY3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key3::W](W) writer structure"]
impl crate::Writable for KEY3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for KEY3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
