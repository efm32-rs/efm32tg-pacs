#[doc = "Register `DATA0XOR` reader"]
pub struct R(crate::R<DATA0XOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0XOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0XOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0XOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0XOR` writer"]
pub struct W(crate::W<DATA0XOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0XOR_SPEC>;
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
impl From<crate::W<DATA0XOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0XOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0XOR` reader - XOR Data 0 Access"]
pub type DATA0XOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA0XOR` writer - XOR Data 0 Access"]
pub type DATA0XOR_W<'a> = crate::FieldWriter<'a, u32, DATA0XOR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&self) -> DATA0XOR_R {
        DATA0XOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&mut self) -> DATA0XOR_W {
        DATA0XOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA0XOR Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0xor](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DATA0XOR_SPEC;
impl crate::RegisterSpec for DATA0XOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0xor::R](R) reader structure"]
impl crate::Readable for DATA0XOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0xor::W](W) writer structure"]
impl crate::Writable for DATA0XOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA0XOR to value 0"]
impl crate::Resettable for DATA0XOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
