#[doc = "Register `DDATA0BIG` reader"]
pub struct R(crate::R<DDATA0BIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDATA0BIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDATA0BIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDATA0BIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDATA0BIG` writer"]
pub struct W(crate::W<DDATA0BIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDATA0BIG_SPEC>;
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
impl From<crate::W<DDATA0BIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDATA0BIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDATA0BIG` reader - Double Data 0 Big Endian Access"]
pub type DDATA0BIG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DDATA0BIG` writer - Double Data 0 Big Endian Access"]
pub type DDATA0BIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDATA0BIG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&self) -> DDATA0BIG_R {
        DDATA0BIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    #[must_use]
    pub fn ddata0big(&mut self) -> DDATA0BIG_W<0> {
        DDATA0BIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDATA0 Register Big Endian Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddata0big](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DDATA0BIG_SPEC;
impl crate::RegisterSpec for DDATA0BIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddata0big::R](R) reader structure"]
impl crate::Readable for DDATA0BIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddata0big::W](W) writer structure"]
impl crate::Writable for DDATA0BIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDATA0BIG to value 0"]
impl crate::Resettable for DDATA0BIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
