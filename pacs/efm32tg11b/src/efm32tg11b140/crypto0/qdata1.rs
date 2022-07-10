#[doc = "Register `QDATA1` reader"]
pub struct R(crate::R<QDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDATA1` writer"]
pub struct W(crate::W<QDATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDATA1_SPEC>;
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
impl From<crate::W<QDATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDATA1` reader - Quad Data 1 Access"]
pub type QDATA1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QDATA1` writer - Quad Data 1 Access"]
pub type QDATA1_W<'a> = crate::FieldWriter<'a, u32, QDATA1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&self) -> QDATA1_R {
        QDATA1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Access"]
    #[inline(always)]
    pub fn qdata1(&mut self) -> QDATA1_W {
        QDATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDATA1 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdata1](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct QDATA1_SPEC;
impl crate::RegisterSpec for QDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdata1::R](R) reader structure"]
impl crate::Readable for QDATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdata1::W](W) writer structure"]
impl crate::Writable for QDATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDATA1 to value 0"]
impl crate::Resettable for QDATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
