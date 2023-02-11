#[doc = "Register `DCDCCTRL` reader"]
pub struct R(crate::R<DCDCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCCTRL` writer"]
pub struct W(crate::W<DCDCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCCTRL_SPEC>;
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
impl From<crate::W<DCDCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDCMODE` reader - Regulator Mode"]
pub type DCDCMODE_R = crate::FieldReader<u8, DCDCMODE_A>;
#[doc = "Regulator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCDCMODE_A {
    #[doc = "0: DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    BYPASS = 0,
    #[doc = "1: DCDC regulator is operating in low noise mode."]
    LOWNOISE = 1,
    #[doc = "2: DCDC regulator is operating in low power mode."]
    LOWPOWER = 2,
    #[doc = "3: DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    OFF = 3,
}
impl From<DCDCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDCMODE_A) -> Self {
        variant as _
    }
}
impl DCDCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDCMODE_A {
        match self.bits {
            0 => DCDCMODE_A::BYPASS,
            1 => DCDCMODE_A::LOWNOISE,
            2 => DCDCMODE_A::LOWPOWER,
            3 => DCDCMODE_A::OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == DCDCMODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `LOWNOISE`"]
    #[inline(always)]
    pub fn is_lownoise(&self) -> bool {
        *self == DCDCMODE_A::LOWNOISE
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == DCDCMODE_A::LOWPOWER
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DCDCMODE_A::OFF
    }
}
#[doc = "Field `DCDCMODE` writer - Regulator Mode"]
pub type DCDCMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCDCCTRL_SPEC, u8, DCDCMODE_A, 2, O>;
impl<'a, const O: u8> DCDCMODE_W<'a, O> {
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DCDCMODE_A::BYPASS)
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline(always)]
    pub fn lownoise(self) -> &'a mut W {
        self.variant(DCDCMODE_A::LOWNOISE)
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(DCDCMODE_A::LOWPOWER)
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DCDCMODE_A::OFF)
    }
}
#[doc = "Field `DCDCMODEEM23` reader - DCDC Mode EM23"]
pub type DCDCMODEEM23_R = crate::BitReader<bool>;
#[doc = "Field `DCDCMODEEM23` writer - DCDC Mode EM23"]
pub type DCDCMODEEM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCCTRL_SPEC, bool, O>;
#[doc = "Field `DCDCMODEEM4` reader - DCDC Mode EM4H"]
pub type DCDCMODEEM4_R = crate::BitReader<bool>;
#[doc = "Field `DCDCMODEEM4` writer - DCDC Mode EM4H"]
pub type DCDCMODEEM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    pub fn dcdcmode(&self) -> DCDCMODE_R {
        DCDCMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    pub fn dcdcmodeem23(&self) -> DCDCMODEEM23_R {
        DCDCMODEEM23_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    pub fn dcdcmodeem4(&self) -> DCDCMODEEM4_R {
        DCDCMODEEM4_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcmode(&mut self) -> DCDCMODE_W<0> {
        DCDCMODE_W::new(self)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcmodeem23(&mut self) -> DCDCMODEEM23_W<4> {
        DCDCMODEEM23_W::new(self)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcmodeem4(&mut self) -> DCDCMODEEM4_W<5> {
        DCDCMODEEM4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcctrl](index.html) module"]
pub struct DCDCCTRL_SPEC;
impl crate::RegisterSpec for DCDCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcctrl::R](R) reader structure"]
impl crate::Readable for DCDCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcctrl::W](W) writer structure"]
impl crate::Writable for DCDCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCCTRL to value 0x33"]
impl crate::Resettable for DCDCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x33;
}
