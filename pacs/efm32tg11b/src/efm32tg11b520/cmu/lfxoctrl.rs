#[doc = "Register `LFXOCTRL` reader"]
pub struct R(crate::R<LFXOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFXOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFXOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFXOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFXOCTRL` writer"]
pub struct W(crate::W<LFXOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFXOCTRL_SPEC>;
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
impl From<crate::W<LFXOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFXOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - LFXO Internal Capacitor Array Tuning Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - LFXO Internal Capacitor Array Tuning Value"]
pub type TUNING_W<'a> = crate::FieldWriter<'a, u32, LFXOCTRL_SPEC, u8, u8, 7, 0>;
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 32768 Hz crystal oscillator"]
    XTAL = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    BUFEXTCLK = 1,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - LFXO Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::XTAL),
            1 => Some(MODE_A::BUFEXTCLK),
            2 => Some(MODE_A::DIGEXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == MODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE_A::DIGEXTCLK
    }
}
#[doc = "Field `MODE` writer - LFXO Mode"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, LFXOCTRL_SPEC, u8, MODE_A, 2, 8>;
impl<'a> MODE_W<'a> {
    #[doc = "32768 Hz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32768 Hz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(MODE_A::BUFEXTCLK)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODE_A::DIGEXTCLK)
    }
}
#[doc = "Field `GAIN` reader - LFXO Startup Gain"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - LFXO Startup Gain"]
pub type GAIN_W<'a> = crate::FieldWriter<'a, u32, LFXOCTRL_SPEC, u8, u8, 2, 11>;
#[doc = "Field `HIGHAMPL` reader - LFXO High XTAL Oscillation Amplitude Enable"]
pub type HIGHAMPL_R = crate::BitReader<bool>;
#[doc = "Field `HIGHAMPL` writer - LFXO High XTAL Oscillation Amplitude Enable"]
pub type HIGHAMPL_W<'a> = crate::BitWriter<'a, u32, LFXOCTRL_SPEC, bool, 14>;
#[doc = "Field `AGC` reader - LFXO AGC Enable"]
pub type AGC_R = crate::BitReader<bool>;
#[doc = "Field `AGC` writer - LFXO AGC Enable"]
pub type AGC_W<'a> = crate::BitWriter<'a, u32, LFXOCTRL_SPEC, bool, 15>;
#[doc = "Field `CUR` reader - LFXO Current Trim"]
pub type CUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUR` writer - LFXO Current Trim"]
pub type CUR_W<'a> = crate::FieldWriter<'a, u32, LFXOCTRL_SPEC, u8, u8, 2, 16>;
#[doc = "Field `BUFCUR` reader - LFXO Buffer Bias Current"]
pub type BUFCUR_R = crate::BitReader<bool>;
#[doc = "Field `BUFCUR` writer - LFXO Buffer Bias Current"]
pub type BUFCUR_W<'a> = crate::BitWriter<'a, u32, LFXOCTRL_SPEC, bool, 20>;
#[doc = "LFXO Timeout\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 256 cycles"]
    _256CYCLES = 1,
    #[doc = "2: Timeout period of 1024 cycles"]
    _1KCYCLES = 2,
    #[doc = "3: Timeout period of 2048 cycles"]
    _2KCYCLES = 3,
    #[doc = "4: Timeout period of 4096 cycles"]
    _4KCYCLES = 4,
    #[doc = "5: Timeout period of 8192 cycles"]
    _8KCYCLES = 5,
    #[doc = "6: Timeout period of 16384 cycles"]
    _16KCYCLES = 6,
    #[doc = "7: Timeout period of 32768 cycles"]
    _32KCYCLES = 7,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEOUT` reader - LFXO Timeout"]
pub type TIMEOUT_R = crate::FieldReader<u8, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            0 => TIMEOUT_A::_2CYCLES,
            1 => TIMEOUT_A::_256CYCLES,
            2 => TIMEOUT_A::_1KCYCLES,
            3 => TIMEOUT_A::_2KCYCLES,
            4 => TIMEOUT_A::_4KCYCLES,
            5 => TIMEOUT_A::_8KCYCLES,
            6 => TIMEOUT_A::_16KCYCLES,
            7 => TIMEOUT_A::_32KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == TIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == TIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_2KCYCLES`"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == TIMEOUT_A::_2KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == TIMEOUT_A::_4KCYCLES
    }
    #[doc = "Checks if the value of the field is `_8KCYCLES`"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == TIMEOUT_A::_8KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == TIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == TIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Field `TIMEOUT` writer - LFXO Timeout"]
pub type TIMEOUT_W<'a> = crate::FieldWriterSafe<'a, u32, LFXOCTRL_SPEC, u8, TIMEOUT_A, 3, 24>;
impl<'a> TIMEOUT_W<'a> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_2KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_4KCYCLES)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_8KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_32KCYCLES)
    }
}
impl R {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&self) -> HIGHAMPL_R {
        HIGHAMPL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&self) -> AGC_R {
        AGC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&self) -> CUR_R {
        CUR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&self) -> BUFCUR_R {
        BUFCUR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFXO Internal Capacitor Array Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W::new(self)
    }
    #[doc = "Bits 8:9 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 11:12 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W::new(self)
    }
    #[doc = "Bit 14 - LFXO High XTAL Oscillation Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&mut self) -> HIGHAMPL_W {
        HIGHAMPL_W::new(self)
    }
    #[doc = "Bit 15 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&mut self) -> AGC_W {
        AGC_W::new(self)
    }
    #[doc = "Bits 16:17 - LFXO Current Trim"]
    #[inline(always)]
    pub fn cur(&mut self) -> CUR_W {
        CUR_W::new(self)
    }
    #[doc = "Bit 20 - LFXO Buffer Bias Current"]
    #[inline(always)]
    pub fn bufcur(&mut self) -> BUFCUR_W {
        BUFCUR_W::new(self)
    }
    #[doc = "Bits 24:26 - LFXO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFXO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfxoctrl](index.html) module"]
pub struct LFXOCTRL_SPEC;
impl crate::RegisterSpec for LFXOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfxoctrl::R](R) reader structure"]
impl crate::Readable for LFXOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfxoctrl::W](W) writer structure"]
impl crate::Writable for LFXOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFXOCTRL to value 0x0700_9000"]
impl crate::Resettable for LFXOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700_9000
    }
}
