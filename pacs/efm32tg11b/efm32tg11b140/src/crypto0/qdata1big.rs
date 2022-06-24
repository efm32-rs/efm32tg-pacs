#[doc = "Register `QDATA1BIG` reader"]
pub struct R(crate::R<QDATA1BIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDATA1BIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDATA1BIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDATA1BIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDATA1BIG` writer"]
pub struct W(crate::W<QDATA1BIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDATA1BIG_SPEC>;
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
impl From<crate::W<QDATA1BIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDATA1BIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDATA1BIG` reader - Quad Data 1 Big Endian Access"]
pub type QDATA1BIG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QDATA1BIG` writer - Quad Data 1 Big Endian Access"]
pub type QDATA1BIG_W<'a> = crate::FieldWriter<'a, u32, QDATA1BIG_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&self) -> QDATA1BIG_R {
        QDATA1BIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&mut self) -> QDATA1BIG_W {
        QDATA1BIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDATA1 Register Big Endian Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdata1big](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct QDATA1BIG_SPEC;
impl crate::RegisterSpec for QDATA1BIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdata1big::R](R) reader structure"]
impl crate::Readable for QDATA1BIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdata1big::W](W) writer structure"]
impl crate::Writable for QDATA1BIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDATA1BIG to value 0"]
impl crate::Resettable for QDATA1BIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
