#[doc = "Register `DDATA0BYTE32` reader"]
pub struct R(crate::R<DDATA0BYTE32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDATA0BYTE32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDATA0BYTE32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDATA0BYTE32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDATA0BYTE32` writer"]
pub struct W(crate::W<DDATA0BYTE32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDATA0BYTE32_SPEC>;
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
impl From<crate::W<DDATA0BYTE32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDATA0BYTE32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDATA0BYTE32` reader - Ddata 0 Byte 32 Access"]
pub type DDATA0BYTE32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDATA0BYTE32` writer - Ddata 0 Byte 32 Access"]
pub type DDATA0BYTE32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDATA0BYTE32_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&self) -> DDATA0BYTE32_R {
        DDATA0BYTE32_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0byte32(&mut self) -> DDATA0BYTE32_W<0> {
        DDATA0BYTE32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDATA0 Register Byte 32 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddata0byte32](index.html) module"]
pub struct DDATA0BYTE32_SPEC;
impl crate::RegisterSpec for DDATA0BYTE32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddata0byte32::R](R) reader structure"]
impl crate::Readable for DDATA0BYTE32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddata0byte32::W](W) writer structure"]
impl crate::Writable for DDATA0BYTE32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDATA0BYTE32 to value 0"]
impl crate::Resettable for DDATA0BYTE32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
