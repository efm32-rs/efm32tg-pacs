#[doc = "Register `DDATA1BYTE` reader"]
pub struct R(crate::R<DDATA1BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDATA1BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDATA1BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDATA1BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDATA1BYTE` writer"]
pub struct W(crate::W<DDATA1BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDATA1BYTE_SPEC>;
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
impl From<crate::W<DDATA1BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDATA1BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDATA1BYTE` reader - Ddata 1 Byte Access"]
pub type DDATA1BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDATA1BYTE` writer - Ddata 1 Byte Access"]
pub type DDATA1BYTE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDATA1BYTE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&self) -> DDATA1BYTE_R {
        DDATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata1byte(&mut self) -> DDATA1BYTE_W<0> {
        DDATA1BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDATA1 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddata1byte](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DDATA1BYTE_SPEC;
impl crate::RegisterSpec for DDATA1BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddata1byte::R](R) reader structure"]
impl crate::Readable for DDATA1BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddata1byte::W](W) writer structure"]
impl crate::Writable for DDATA1BYTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDATA1BYTE to value 0"]
impl crate::Resettable for DDATA1BYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
