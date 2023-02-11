#[doc = "Register `DATA0BYTE12` reader"]
pub struct R(crate::R<DATA0BYTE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0BYTE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0BYTE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0BYTE12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0BYTE12` writer"]
pub struct W(crate::W<DATA0BYTE12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0BYTE12_SPEC>;
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
impl From<crate::W<DATA0BYTE12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0BYTE12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0BYTE12` reader - Data 0 Byte 12 Access"]
pub type DATA0BYTE12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0BYTE12` writer - Data 0 Byte 12 Access"]
pub type DATA0BYTE12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA0BYTE12_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    pub fn data0byte12(&self) -> DATA0BYTE12_R {
        DATA0BYTE12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte12(&mut self) -> DATA0BYTE12_W<0> {
        DATA0BYTE12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA0 Register Byte 12 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0byte12](index.html) module"]
pub struct DATA0BYTE12_SPEC;
impl crate::RegisterSpec for DATA0BYTE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0byte12::R](R) reader structure"]
impl crate::Readable for DATA0BYTE12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0byte12::W](W) writer structure"]
impl crate::Writable for DATA0BYTE12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0BYTE12 to value 0"]
impl crate::Resettable for DATA0BYTE12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
