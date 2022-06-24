#[doc = "Register `DCDCLPCTRL` reader"]
pub struct R(crate::R<DCDCLPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCLPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCLPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCLPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCLPCTRL` writer"]
pub struct W(crate::W<DCDCLPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCLPCTRL_SPEC>;
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
impl From<crate::W<DCDCLPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCLPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCMPHYSSELEM234H` reader - LP Mode Hysteresis Selection for EM23 and EM4H"]
pub type LPCMPHYSSELEM234H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPCMPHYSSELEM234H` writer - LP Mode Hysteresis Selection for EM23 and EM4H"]
pub type LPCMPHYSSELEM234H_W<'a> = crate::FieldWriter<'a, u32, DCDCLPCTRL_SPEC, u8, u8, 4, 12>;
#[doc = "Field `LPVREFDUTYEN` reader - LP Mode Duty Cycling Enable"]
pub type LPVREFDUTYEN_R = crate::BitReader<bool>;
#[doc = "Field `LPVREFDUTYEN` writer - LP Mode Duty Cycling Enable"]
pub type LPVREFDUTYEN_W<'a> = crate::BitWriter<'a, u32, DCDCLPCTRL_SPEC, bool, 24>;
#[doc = "Field `LPBLANK` reader - Reserved for internal use. Do not change."]
pub type LPBLANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBLANK` writer - Reserved for internal use. Do not change."]
pub type LPBLANK_W<'a> = crate::FieldWriter<'a, u32, DCDCLPCTRL_SPEC, u8, u8, 2, 25>;
impl R {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpcmphysselem234h(&self) -> LPCMPHYSSELEM234H_R {
        LPCMPHYSSELEM234H_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&self) -> LPVREFDUTYEN_R {
        LPVREFDUTYEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&self) -> LPBLANK_R {
        LPBLANK_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpcmphysselem234h(&mut self) -> LPCMPHYSSELEM234H_W {
        LPCMPHYSSELEM234H_W::new(self)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&mut self) -> LPVREFDUTYEN_W {
        LPVREFDUTYEN_W::new(self)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&mut self) -> LPBLANK_W {
        LPBLANK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclpctrl](index.html) module"]
pub struct DCDCLPCTRL_SPEC;
impl crate::RegisterSpec for DCDCLPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdclpctrl::R](R) reader structure"]
impl crate::Readable for DCDCLPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdclpctrl::W](W) writer structure"]
impl crate::Writable for DCDCLPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCLPCTRL to value 0x0300_0000"]
impl crate::Resettable for DCDCLPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
