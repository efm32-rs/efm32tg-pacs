#[doc = "Register `QDATA0` reader"]
pub struct R(crate::R<QDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDATA0` writer"]
pub struct W(crate::W<QDATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDATA0_SPEC>;
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
impl From<crate::W<QDATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDATA0` reader - Quad Data 0 Access"]
pub type QDATA0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QDATA0` writer - Quad Data 0 Access"]
pub type QDATA0_W<'a> = crate::FieldWriter<'a, u32, QDATA0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&self) -> QDATA0_R {
        QDATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&mut self) -> QDATA0_W {
        QDATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDATA0 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdata0](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct QDATA0_SPEC;
impl crate::RegisterSpec for QDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdata0::R](R) reader structure"]
impl crate::Readable for QDATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdata0::W](W) writer structure"]
impl crate::Writable for QDATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDATA0 to value 0"]
impl crate::Resettable for QDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
