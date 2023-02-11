#[doc = "Register `DATA0XORBYTE` reader"]
pub struct R(crate::R<DATA0XORBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0XORBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0XORBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0XORBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0XORBYTE` writer"]
pub struct W(crate::W<DATA0XORBYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0XORBYTE_SPEC>;
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
impl From<crate::W<DATA0XORBYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0XORBYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0XORBYTE` reader - Data 0 XOR Byte Access"]
pub type DATA0XORBYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0XORBYTE` writer - Data 0 XOR Byte Access"]
pub type DATA0XORBYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA0XORBYTE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&self) -> DATA0XORBYTE_R {
        DATA0XORBYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0xorbyte(&mut self) -> DATA0XORBYTE_W<0> {
        DATA0XORBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA0 Register Byte XOR Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0xorbyte](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DATA0XORBYTE_SPEC;
impl crate::RegisterSpec for DATA0XORBYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0xorbyte::R](R) reader structure"]
impl crate::Readable for DATA0XORBYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0xorbyte::W](W) writer structure"]
impl crate::Writable for DATA0XORBYTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0XORBYTE to value 0"]
impl crate::Resettable for DATA0XORBYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
