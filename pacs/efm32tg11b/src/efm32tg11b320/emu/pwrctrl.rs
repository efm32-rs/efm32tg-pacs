#[doc = "Register `PWRCTRL` reader"]
pub struct R(crate::R<PWRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTRL` writer"]
pub struct W(crate::W<PWRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTRL_SPEC>;
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
impl From<crate::W<PWRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANASW` reader - Analog Switch Selection"]
pub type ANASW_R = crate::BitReader<bool>;
#[doc = "Field `ANASW` writer - Analog Switch Selection"]
pub type ANASW_W<'a> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, 5>;
#[doc = "Field `REGPWRSEL` reader - This Field Selects the Input Supply Pin for the Digital LDO"]
pub type REGPWRSEL_R = crate::BitReader<bool>;
#[doc = "Field `REGPWRSEL` writer - This Field Selects the Input Supply Pin for the Digital LDO"]
pub type REGPWRSEL_W<'a> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, 10>;
#[doc = "Field `IMMEDIATEPWRSWITCH` reader - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
pub type IMMEDIATEPWRSWITCH_R = crate::BitReader<bool>;
#[doc = "Field `IMMEDIATEPWRSWITCH` writer - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
pub type IMMEDIATEPWRSWITCH_W<'a> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&self) -> ANASW_R {
        ANASW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - This Field Selects the Input Supply Pin for the Digital LDO"]
    #[inline(always)]
    pub fn regpwrsel(&self) -> REGPWRSEL_R {
        REGPWRSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
    #[inline(always)]
    pub fn immediatepwrswitch(&self) -> IMMEDIATEPWRSWITCH_R {
        IMMEDIATEPWRSWITCH_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&mut self) -> ANASW_W {
        ANASW_W::new(self)
    }
    #[doc = "Bit 10 - This Field Selects the Input Supply Pin for the Digital LDO"]
    #[inline(always)]
    pub fn regpwrsel(&mut self) -> REGPWRSEL_W {
        REGPWRSEL_W::new(self)
    }
    #[doc = "Bit 13 - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
    #[inline(always)]
    pub fn immediatepwrswitch(&mut self) -> IMMEDIATEPWRSWITCH_W {
        IMMEDIATEPWRSWITCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](index.html) module"]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctrl::R](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PWRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
