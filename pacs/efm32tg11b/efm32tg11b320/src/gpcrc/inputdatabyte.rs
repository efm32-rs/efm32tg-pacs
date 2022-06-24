#[doc = "Register `INPUTDATABYTE` reader"]
pub struct R(crate::R<INPUTDATABYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTDATABYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTDATABYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTDATABYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTDATABYTE` writer"]
pub struct W(crate::W<INPUTDATABYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTDATABYTE_SPEC>;
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
impl From<crate::W<INPUTDATABYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTDATABYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTDATABYTE` reader - Input Data for 8-bit"]
pub type INPUTDATABYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUTDATABYTE` writer - Input Data for 8-bit"]
pub type INPUTDATABYTE_W<'a> = crate::FieldWriter<'a, u32, INPUTDATABYTE_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&self) -> INPUTDATABYTE_R {
        INPUTDATABYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&mut self) -> INPUTDATABYTE_W {
        INPUTDATABYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input 8-bit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdatabyte](index.html) module"]
pub struct INPUTDATABYTE_SPEC;
impl crate::RegisterSpec for INPUTDATABYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputdatabyte::R](R) reader structure"]
impl crate::Readable for INPUTDATABYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputdatabyte::W](W) writer structure"]
impl crate::Writable for INPUTDATABYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUTDATABYTE to value 0"]
impl crate::Resettable for INPUTDATABYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
