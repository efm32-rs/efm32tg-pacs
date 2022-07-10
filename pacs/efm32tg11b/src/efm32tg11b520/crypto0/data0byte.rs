#[doc = "Register `DATA0BYTE` reader"]
pub struct R(crate::R<DATA0BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0BYTE` writer"]
pub struct W(crate::W<DATA0BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0BYTE_SPEC>;
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
impl From<crate::W<DATA0BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0BYTE` reader - Data 0 Byte Access"]
pub type DATA0BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0BYTE` writer - Data 0 Byte Access"]
pub type DATA0BYTE_W<'a> = crate::FieldWriter<'a, u32, DATA0BYTE_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&self) -> DATA0BYTE_R {
        DATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&mut self) -> DATA0BYTE_W {
        DATA0BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA0 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0byte](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DATA0BYTE_SPEC;
impl crate::RegisterSpec for DATA0BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0byte::R](R) reader structure"]
impl crate::Readable for DATA0BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0byte::W](W) writer structure"]
impl crate::Writable for DATA0BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA0BYTE to value 0"]
impl crate::Resettable for DATA0BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
