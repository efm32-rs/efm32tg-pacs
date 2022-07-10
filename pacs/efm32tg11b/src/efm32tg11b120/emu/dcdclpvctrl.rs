#[doc = "Register `DCDCLPVCTRL` reader"]
pub struct R(crate::R<DCDCLPVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCLPVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCLPVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCLPVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCLPVCTRL` writer"]
pub struct W(crate::W<DCDCLPVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCLPVCTRL_SPEC>;
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
impl From<crate::W<DCDCLPVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCLPVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPATT` reader - Low Power Feedback Attenuation"]
pub type LPATT_R = crate::BitReader<bool>;
#[doc = "Field `LPATT` writer - Low Power Feedback Attenuation"]
pub type LPATT_W<'a> = crate::BitWriter<'a, u32, DCDCLPVCTRL_SPEC, bool, 0>;
#[doc = "Field `LPVREF` reader - LP Mode Reference Selection for EM23 and EM4H"]
pub type LPVREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPVREF` writer - LP Mode Reference Selection for EM23 and EM4H"]
pub type LPVREF_W<'a> = crate::FieldWriter<'a, u32, DCDCLPVCTRL_SPEC, u8, u8, 8, 1>;
impl R {
    #[doc = "Bit 0 - Low Power Feedback Attenuation"]
    #[inline(always)]
    pub fn lpatt(&self) -> LPATT_R {
        LPATT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpvref(&self) -> LPVREF_R {
        LPVREF_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Feedback Attenuation"]
    #[inline(always)]
    pub fn lpatt(&mut self) -> LPATT_W {
        LPATT_W::new(self)
    }
    #[doc = "Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpvref(&mut self) -> LPVREF_W {
        LPVREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Low Power Voltage Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclpvctrl](index.html) module"]
pub struct DCDCLPVCTRL_SPEC;
impl crate::RegisterSpec for DCDCLPVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdclpvctrl::R](R) reader structure"]
impl crate::Readable for DCDCLPVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdclpvctrl::W](W) writer structure"]
impl crate::Writable for DCDCLPVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCLPVCTRL to value 0x0168"]
impl crate::Resettable for DCDCLPVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0168
    }
}
