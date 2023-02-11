#[doc = "Register `DATA0BYTE14` reader"]
pub struct R(crate::R<DATA0BYTE14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0BYTE14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0BYTE14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0BYTE14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0BYTE14` writer"]
pub struct W(crate::W<DATA0BYTE14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0BYTE14_SPEC>;
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
impl From<crate::W<DATA0BYTE14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0BYTE14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0BYTE14` reader - Data 0 Byte 14 Access"]
pub type DATA0BYTE14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0BYTE14` writer - Data 0 Byte 14 Access"]
pub type DATA0BYTE14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA0BYTE14_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    pub fn data0byte14(&self) -> DATA0BYTE14_R {
        DATA0BYTE14_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte14(&mut self) -> DATA0BYTE14_W<0> {
        DATA0BYTE14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA0 Register Byte 14 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0byte14](index.html) module"]
pub struct DATA0BYTE14_SPEC;
impl crate::RegisterSpec for DATA0BYTE14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0byte14::R](R) reader structure"]
impl crate::Readable for DATA0BYTE14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0byte14::W](W) writer structure"]
impl crate::Writable for DATA0BYTE14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0BYTE14 to value 0"]
impl crate::Resettable for DATA0BYTE14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
