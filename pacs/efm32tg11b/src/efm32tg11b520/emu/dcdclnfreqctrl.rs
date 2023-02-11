#[doc = "Register `DCDCLNFREQCTRL` reader"]
pub struct R(crate::R<DCDCLNFREQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCLNFREQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCLNFREQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCLNFREQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCLNFREQCTRL` writer"]
pub struct W(crate::W<DCDCLNFREQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCLNFREQCTRL_SPEC>;
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
impl From<crate::W<DCDCLNFREQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCLNFREQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOBAND` reader - LN Mode RCO Frequency Band Selection"]
pub type RCOBAND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOBAND` writer - LN Mode RCO Frequency Band Selection"]
pub type RCOBAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNFREQCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RCOTRIM` reader - Reserved for internal use. Do not change."]
pub type RCOTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOTRIM` writer - Reserved for internal use. Do not change."]
pub type RCOTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNFREQCTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:2 - LN Mode RCO Frequency Band Selection"]
    #[inline(always)]
    pub fn rcoband(&self) -> RCOBAND_R {
        RCOBAND_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 24:28 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn rcotrim(&self) -> RCOTRIM_R {
        RCOTRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LN Mode RCO Frequency Band Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rcoband(&mut self) -> RCOBAND_W<0> {
        RCOBAND_W::new(self)
    }
    #[doc = "Bits 24:28 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn rcotrim(&mut self) -> RCOTRIM_W<24> {
        RCOTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Low Noise Controller Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclnfreqctrl](index.html) module"]
pub struct DCDCLNFREQCTRL_SPEC;
impl crate::RegisterSpec for DCDCLNFREQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdclnfreqctrl::R](R) reader structure"]
impl crate::Readable for DCDCLNFREQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdclnfreqctrl::W](W) writer structure"]
impl crate::Writable for DCDCLNFREQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCLNFREQCTRL to value 0x1000_0000"]
impl crate::Resettable for DCDCLNFREQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
