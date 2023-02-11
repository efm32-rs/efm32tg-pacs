#[doc = "Register `DCDCZDETCTRL` reader"]
pub struct R(crate::R<DCDCZDETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCZDETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCZDETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCZDETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCZDETCTRL` writer"]
pub struct W(crate::W<DCDCZDETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCZDETCTRL_SPEC>;
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
impl From<crate::W<DCDCZDETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCZDETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZDETILIMSEL` reader - Reverse Current Limit Level Selection for Zero Detector"]
pub type ZDETILIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZDETILIMSEL` writer - Reverse Current Limit Level Selection for Zero Detector"]
pub type ZDETILIMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCZDETCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `ZDETBLANKDLY` reader - Reserved for internal use. Do not change."]
pub type ZDETBLANKDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZDETBLANKDLY` writer - Reserved for internal use. Do not change."]
pub type ZDETBLANKDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCZDETCTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    pub fn zdetilimsel(&self) -> ZDETILIMSEL_R {
        ZDETILIMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn zdetblankdly(&self) -> ZDETBLANKDLY_R {
        ZDETBLANKDLY_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    #[must_use]
    pub fn zdetilimsel(&mut self) -> ZDETILIMSEL_W<4> {
        ZDETILIMSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn zdetblankdly(&mut self) -> ZDETBLANKDLY_W<8> {
        ZDETBLANKDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdczdetctrl](index.html) module"]
pub struct DCDCZDETCTRL_SPEC;
impl crate::RegisterSpec for DCDCZDETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdczdetctrl::R](R) reader structure"]
impl crate::Readable for DCDCZDETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdczdetctrl::W](W) writer structure"]
impl crate::Writable for DCDCZDETCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCZDETCTRL to value 0x0150"]
impl crate::Resettable for DCDCZDETCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0150;
}
