#[doc = "Register `EM4WUEN` reader"]
pub struct R(crate::R<EM4WUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4WUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4WUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4WUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4WUEN` writer"]
pub struct W(crate::W<EM4WUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4WUEN_SPEC>;
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
impl From<crate::W<EM4WUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4WUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4WUEN` reader - EM4 Wake Up Enable"]
pub type EM4WUEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EM4WUEN` writer - EM4 Wake Up Enable"]
pub type EM4WUEN_W<'a> = crate::FieldWriter<'a, u32, EM4WUEN_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 16:31 - EM4 Wake Up Enable"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - EM4 Wake Up Enable"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> EM4WUEN_W {
        EM4WUEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EM4 Wake Up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wuen](index.html) module"]
pub struct EM4WUEN_SPEC;
impl crate::RegisterSpec for EM4WUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4wuen::R](R) reader structure"]
impl crate::Readable for EM4WUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4wuen::W](W) writer structure"]
impl crate::Writable for EM4WUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
