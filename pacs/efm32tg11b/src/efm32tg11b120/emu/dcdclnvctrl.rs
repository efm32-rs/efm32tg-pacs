#[doc = "Register `DCDCLNVCTRL` reader"]
pub struct R(crate::R<DCDCLNVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCLNVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCLNVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCLNVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCLNVCTRL` writer"]
pub struct W(crate::W<DCDCLNVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCLNVCTRL_SPEC>;
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
impl From<crate::W<DCDCLNVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCLNVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNATT` reader - Low Noise Mode Feedback Attenuation"]
pub type LNATT_R = crate::BitReader<bool>;
#[doc = "Field `LNATT` writer - Low Noise Mode Feedback Attenuation"]
pub type LNATT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCLNVCTRL_SPEC, bool, O>;
#[doc = "Field `LNVREF` reader - Low Noise Mode VREF Trim"]
pub type LNVREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNVREF` writer - Low Noise Mode VREF Trim"]
pub type LNVREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCLNVCTRL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    pub fn lnatt(&self) -> LNATT_R {
        LNATT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    pub fn lnvref(&self) -> LNVREF_R {
        LNVREF_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    #[must_use]
    pub fn lnatt(&mut self) -> LNATT_W<1> {
        LNATT_W::new(self)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    #[must_use]
    pub fn lnvref(&mut self) -> LNVREF_W<8> {
        LNVREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Low Noise Voltage Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclnvctrl](index.html) module"]
pub struct DCDCLNVCTRL_SPEC;
impl crate::RegisterSpec for DCDCLNVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdclnvctrl::R](R) reader structure"]
impl crate::Readable for DCDCLNVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdclnvctrl::W](W) writer structure"]
impl crate::Writable for DCDCLNVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCLNVCTRL to value 0x7100"]
impl crate::Resettable for DCDCLNVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7100;
}
