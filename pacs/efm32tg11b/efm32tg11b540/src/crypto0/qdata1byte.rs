#[doc = "Register `QDATA1BYTE` reader"]
pub struct R(crate::R<QDATA1BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDATA1BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDATA1BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDATA1BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDATA1BYTE` writer"]
pub struct W(crate::W<QDATA1BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDATA1BYTE_SPEC>;
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
impl From<crate::W<QDATA1BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDATA1BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDATA1BYTE` reader - Qdata 1 Byte Access"]
pub type QDATA1BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QDATA1BYTE` writer - Qdata 1 Byte Access"]
pub type QDATA1BYTE_W<'a> = crate::FieldWriter<'a, u32, QDATA1BYTE_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&self) -> QDATA1BYTE_R {
        QDATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&mut self) -> QDATA1BYTE_W {
        QDATA1BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDATA1 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdata1byte](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct QDATA1BYTE_SPEC;
impl crate::RegisterSpec for QDATA1BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdata1byte::R](R) reader structure"]
impl crate::Readable for QDATA1BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdata1byte::W](W) writer structure"]
impl crate::Writable for QDATA1BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDATA1BYTE to value 0"]
impl crate::Resettable for QDATA1BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
