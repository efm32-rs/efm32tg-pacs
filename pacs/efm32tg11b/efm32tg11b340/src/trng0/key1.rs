#[doc = "Register `KEY1` reader"]
pub struct R(crate::R<KEY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY1` writer"]
pub struct W(crate::W<KEY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY1_SPEC>;
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
impl From<crate::W<KEY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Key 1"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Key 1"]
pub type VALUE_W<'a> = crate::FieldWriter<'a, u32, KEY1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key 1"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key 1"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](index.html) module"]
pub struct KEY1_SPEC;
impl crate::RegisterSpec for KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key1::R](R) reader structure"]
impl crate::Readable for KEY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key1::W](W) writer structure"]
impl crate::Writable for KEY1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for KEY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
