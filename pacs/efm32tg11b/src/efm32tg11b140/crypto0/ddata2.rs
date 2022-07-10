#[doc = "Register `DDATA2` reader"]
pub struct R(crate::R<DDATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDATA2` writer"]
pub struct W(crate::W<DDATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDATA2_SPEC>;
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
impl From<crate::W<DDATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDATA2` reader - Double Data 0 Access"]
pub type DDATA2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DDATA2` writer - Double Data 0 Access"]
pub type DDATA2_W<'a> = crate::FieldWriter<'a, u32, DDATA2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&self) -> DDATA2_R {
        DDATA2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&mut self) -> DDATA2_W {
        DDATA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDATA2 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddata2](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DDATA2_SPEC;
impl crate::RegisterSpec for DDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddata2::R](R) reader structure"]
impl crate::Readable for DDATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddata2::W](W) writer structure"]
impl crate::Writable for DDATA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDATA2 to value 0"]
impl crate::Resettable for DDATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
