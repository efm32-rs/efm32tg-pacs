#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `SINEMODE` reader - Sine Mode"]
pub type SINEMODE_R = crate::BitReader<bool>;
#[doc = "Field `SINEMODE` writer - Sine Mode"]
pub type SINEMODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `OUTENPRS` reader - PRS Controlled Output Enable"]
pub type OUTENPRS_R = crate::BitReader<bool>;
#[doc = "Field `OUTENPRS` writer - PRS Controlled Output Enable"]
pub type OUTENPRS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `CH0PRESCRST` reader - Channel 0 Start Reset Prescaler"]
pub type CH0PRESCRST_R = crate::BitReader<bool>;
#[doc = "Field `CH0PRESCRST` writer - Channel 0 Start Reset Prescaler"]
pub type CH0PRESCRST_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal low noise 1.25 V bandgap reference"]
    _1V25LN = 0,
    #[doc = "1: Internal low noise 2.5 V bandgap reference"]
    _2V5LN = 1,
    #[doc = "2: Internal 1.25 V bandgap reference"]
    _1V25 = 2,
    #[doc = "3: Internal 2.5 V bandgap reference"]
    _2V5 = 3,
    #[doc = "4: AVDD reference"]
    VDD = 4,
    #[doc = "6: External pin reference"]
    EXT = 6,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::_1V25LN),
            1 => Some(REFSEL_A::_2V5LN),
            2 => Some(REFSEL_A::_1V25),
            3 => Some(REFSEL_A::_2V5),
            4 => Some(REFSEL_A::VDD),
            6 => Some(REFSEL_A::EXT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1V25LN`"]
    #[inline(always)]
    pub fn is_1v25ln(&self) -> bool {
        *self == REFSEL_A::_1V25LN
    }
    #[doc = "Checks if the value of the field is `_2V5LN`"]
    #[inline(always)]
    pub fn is_2v5ln(&self) -> bool {
        *self == REFSEL_A::_2V5LN
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REFSEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REFSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == REFSEL_A::EXT
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, REFSEL_A, 3, 8>;
impl<'a> REFSEL_W<'a> {
    #[doc = "Internal low noise 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25ln(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V25LN)
    }
    #[doc = "Internal low noise 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5ln(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V5LN)
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REFSEL_A::_1V25)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REFSEL_A::_2V5)
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(REFSEL_A::EXT)
    }
}
#[doc = "Prescaler Setting for DAC Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting for DAC Clock"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting for DAC Clock"]
pub type PRESC_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRESC_A, 7, 16>;
impl<'a> PRESC_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "Refresh Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFRESHPERIOD_A {
    #[doc = "0: All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    _8CYCLES = 0,
    #[doc = "1: All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    _16CYCLES = 1,
    #[doc = "2: All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    _32CYCLES = 2,
    #[doc = "3: All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    _64CYCLES = 3,
}
impl From<REFRESHPERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: REFRESHPERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFRESHPERIOD` reader - Refresh Period"]
pub type REFRESHPERIOD_R = crate::FieldReader<u8, REFRESHPERIOD_A>;
impl REFRESHPERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFRESHPERIOD_A {
        match self.bits {
            0 => REFRESHPERIOD_A::_8CYCLES,
            1 => REFRESHPERIOD_A::_16CYCLES,
            2 => REFRESHPERIOD_A::_32CYCLES,
            3 => REFRESHPERIOD_A::_64CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == REFRESHPERIOD_A::_64CYCLES
    }
}
#[doc = "Field `REFRESHPERIOD` writer - Refresh Period"]
pub type REFRESHPERIOD_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, REFRESHPERIOD_A, 2, 24>;
impl<'a> REFRESHPERIOD_W<'a> {
    #[doc = "All channels with enabled refresh are refreshed every 8 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_8CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_16CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_32CYCLES)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 DAC_CLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REFRESHPERIOD_A::_64CYCLES)
    }
}
#[doc = "Field `WARMUPMODE` reader - Warm-up Mode"]
pub type WARMUPMODE_R = crate::BitReader<bool>;
#[doc = "Field `WARMUPMODE` writer - Warm-up Mode"]
pub type WARMUPMODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 28>;
#[doc = "Field `DACCLKMODE` reader - Clock Mode"]
pub type DACCLKMODE_R = crate::BitReader<bool>;
#[doc = "Field `DACCLKMODE` writer - Clock Mode"]
pub type DACCLKMODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&self) -> SINEMODE_R {
        SINEMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OUTENPRS_R {
        OUTENPRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&self) -> CH0PRESCRST_R {
        CH0PRESCRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline(always)]
    pub fn refreshperiod(&self) -> REFRESHPERIOD_R {
        REFRESHPERIOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline(always)]
    pub fn dacclkmode(&self) -> DACCLKMODE_R {
        DACCLKMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W::new(self)
    }
    #[doc = "Bit 4 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&mut self) -> SINEMODE_W {
        SINEMODE_W::new(self)
    }
    #[doc = "Bit 5 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&mut self) -> OUTENPRS_W {
        OUTENPRS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&mut self) -> CH0PRESCRST_W {
        CH0PRESCRST_W::new(self)
    }
    #[doc = "Bits 8:10 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 16:22 - Prescaler Setting for DAC Clock"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Bits 24:25 - Refresh Period"]
    #[inline(always)]
    pub fn refreshperiod(&mut self) -> REFRESHPERIOD_W {
        REFRESHPERIOD_W::new(self)
    }
    #[doc = "Bit 28 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W {
        WARMUPMODE_W::new(self)
    }
    #[doc = "Bit 31 - Clock Mode"]
    #[inline(always)]
    pub fn dacclkmode(&mut self) -> DACCLKMODE_W {
        DACCLKMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
