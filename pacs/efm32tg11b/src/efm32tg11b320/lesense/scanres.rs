#[doc = "Register `SCANRES` reader"]
pub struct R(crate::R<SCANRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANRES` writer"]
pub struct W(crate::W<SCANRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANRES_SPEC>;
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
impl From<crate::W<SCANRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANRES` reader - Scan Results"]
pub type SCANRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCANRES` writer - Scan Results"]
pub type SCANRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCANRES_SPEC, u16, u16, 16, O>;
#[doc = "Field `STEPDIR` reader - Direction of Previous Step Detection"]
pub type STEPDIR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STEPDIR` writer - Direction of Previous Step Detection"]
pub type STEPDIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCANRES_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    pub fn scanres(&self) -> SCANRES_R {
        SCANRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    pub fn stepdir(&self) -> STEPDIR_R {
        STEPDIR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    #[must_use]
    pub fn scanres(&mut self) -> SCANRES_W<0> {
        SCANRES_W::new(self)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    #[must_use]
    pub fn stepdir(&mut self) -> STEPDIR_W<16> {
        STEPDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanres](index.html) module"]
pub struct SCANRES_SPEC;
impl crate::RegisterSpec for SCANRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanres::R](R) reader structure"]
impl crate::Readable for SCANRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanres::W](W) writer structure"]
impl crate::Writable for SCANRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANRES to value 0"]
impl crate::Resettable for SCANRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
