#[doc = "Register `PK_OVTDIS` reader"]
pub struct R(crate::R<PK_OVTDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PK_OVTDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PK_OVTDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PK_OVTDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PK_OVTDIS` writer"]
pub struct W(crate::W<PK_OVTDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PK_OVTDIS_SPEC>;
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
impl From<crate::W<PK_OVTDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PK_OVTDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVTDIS` reader - Disable Over Voltage Capability"]
pub type OVTDIS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVTDIS` writer - Disable Over Voltage Capability"]
pub type OVTDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PK_OVTDIS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OVTDIS_R {
        OVTDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    #[must_use]
    pub fn ovtdis(&mut self) -> OVTDIS_W<0> {
        OVTDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_ovtdis](index.html) module"]
pub struct PK_OVTDIS_SPEC;
impl crate::RegisterSpec for PK_OVTDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pk_ovtdis::R](R) reader structure"]
impl crate::Readable for PK_OVTDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pk_ovtdis::W](W) writer structure"]
impl crate::Writable for PK_OVTDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PK_OVTDIS to value 0"]
impl crate::Resettable for PK_OVTDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
