#[doc = "Register `INPUTDATA` reader"]
pub struct R(crate::R<INPUTDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTDATA` writer"]
pub struct W(crate::W<INPUTDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTDATA_SPEC>;
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
impl From<crate::W<INPUTDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTDATA` reader - Input Data for 32-bit"]
pub type INPUTDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INPUTDATA` writer - Input Data for 32-bit"]
pub type INPUTDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INPUTDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&self) -> INPUTDATA_R {
        INPUTDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdata(&mut self) -> INPUTDATA_W<0> {
        INPUTDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input 32-bit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdata](index.html) module"]
pub struct INPUTDATA_SPEC;
impl crate::RegisterSpec for INPUTDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputdata::R](R) reader structure"]
impl crate::Readable for INPUTDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputdata::W](W) writer structure"]
impl crate::Writable for INPUTDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTDATA to value 0"]
impl crate::Resettable for INPUTDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
