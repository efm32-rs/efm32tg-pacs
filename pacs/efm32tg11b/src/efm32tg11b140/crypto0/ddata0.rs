#[doc = "Register `DDATA0` reader"]
pub struct R(crate::R<DDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDATA0` writer"]
pub struct W(crate::W<DDATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDATA0_SPEC>;
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
impl From<crate::W<DDATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDATA0` reader - Double Data 0 Access"]
pub type DDATA0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DDATA0` writer - Double Data 0 Access"]
pub type DDATA0_W<'a> = crate::FieldWriter<'a, u32, DDATA0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata0(&self) -> DDATA0_R {
        DDATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata0(&mut self) -> DDATA0_W {
        DDATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDATA0 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddata0](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DDATA0_SPEC;
impl crate::RegisterSpec for DDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddata0::R](R) reader structure"]
impl crate::Readable for DDATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddata0::W](W) writer structure"]
impl crate::Writable for DDATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDATA0 to value 0"]
impl crate::Resettable for DDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
