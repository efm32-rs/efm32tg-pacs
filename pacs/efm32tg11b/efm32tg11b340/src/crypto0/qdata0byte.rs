#[doc = "Register `QDATA0BYTE` reader"]
pub struct R(crate::R<QDATA0BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDATA0BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDATA0BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDATA0BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDATA0BYTE` writer"]
pub struct W(crate::W<QDATA0BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDATA0BYTE_SPEC>;
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
impl From<crate::W<QDATA0BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDATA0BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDATA0BYTE` reader - Qdata 0 Byte Access"]
pub type QDATA0BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QDATA0BYTE` writer - Qdata 0 Byte Access"]
pub type QDATA0BYTE_W<'a> = crate::FieldWriter<'a, u32, QDATA0BYTE_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    pub fn qdata0byte(&self) -> QDATA0BYTE_R {
        QDATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    pub fn qdata0byte(&mut self) -> QDATA0BYTE_W {
        QDATA0BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDATA0 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdata0byte](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct QDATA0BYTE_SPEC;
impl crate::RegisterSpec for QDATA0BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdata0byte::R](R) reader structure"]
impl crate::Readable for QDATA0BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdata0byte::W](W) writer structure"]
impl crate::Writable for QDATA0BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDATA0BYTE to value 0"]
impl crate::Resettable for QDATA0BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
