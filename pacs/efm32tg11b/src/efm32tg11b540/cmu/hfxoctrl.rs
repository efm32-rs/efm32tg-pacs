#[doc = "Register `HFXOCTRL` reader"]
pub struct R(crate::R<HFXOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOCTRL` writer"]
pub struct W(crate::W<HFXOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOCTRL_SPEC>;
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
impl From<crate::W<HFXOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 4 MHz - 48 MHz crystal oscillator"]
    XTAL = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    ACBUFEXTCLK = 1,
    #[doc = "2: A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    DCBUFEXTCLK = 2,
    #[doc = "3: Digital external clock can be supplied on HFXTAL_N pin."]
    DIGEXTCLK = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - HFXO Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::XTAL,
            1 => MODE_A::ACBUFEXTCLK,
            2 => MODE_A::DCBUFEXTCLK,
            3 => MODE_A::DIGEXTCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == MODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `ACBUFEXTCLK`"]
    #[inline(always)]
    pub fn is_acbufextclk(&self) -> bool {
        *self == MODE_A::ACBUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DCBUFEXTCLK`"]
    #[inline(always)]
    pub fn is_dcbufextclk(&self) -> bool {
        *self == MODE_A::DCBUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == MODE_A::DIGEXTCLK
    }
}
#[doc = "Field `MODE` writer - HFXO Mode"]
pub type MODE_W<'a> = crate::FieldWriterSafe<'a, u32, HFXOCTRL_SPEC, u8, MODE_A, 2, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "4 MHz - 48 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(MODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn acbufextclk(self) -> &'a mut W {
        self.variant(MODE_A::ACBUFEXTCLK)
    }
    #[doc = "A DC coupled buffer is coupled in series with HFXTAL_N pin, suitable for external sinus wave."]
    #[inline(always)]
    pub fn dcbufextclk(self) -> &'a mut W {
        self.variant(MODE_A::DCBUFEXTCLK)
    }
    #[doc = "Digital external clock can be supplied on HFXTAL_N pin."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(MODE_A::DIGEXTCLK)
    }
}
#[doc = "HFXO Automatic Peak Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETMODE_A {
    #[doc = "0: Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    ONCECMD = 0,
    #[doc = "1: Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    AUTOCMD = 1,
    #[doc = "2: CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    CMD = 2,
    #[doc = "3: CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    MANUAL = 3,
}
impl From<PEAKDETMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEAKDETMODE` reader - HFXO Automatic Peak Detection Mode"]
pub type PEAKDETMODE_R = crate::FieldReader<u8, PEAKDETMODE_A>;
impl PEAKDETMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEAKDETMODE_A {
        match self.bits {
            0 => PEAKDETMODE_A::ONCECMD,
            1 => PEAKDETMODE_A::AUTOCMD,
            2 => PEAKDETMODE_A::CMD,
            3 => PEAKDETMODE_A::MANUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONCECMD`"]
    #[inline(always)]
    pub fn is_oncecmd(&self) -> bool {
        *self == PEAKDETMODE_A::ONCECMD
    }
    #[doc = "Checks if the value of the field is `AUTOCMD`"]
    #[inline(always)]
    pub fn is_autocmd(&self) -> bool {
        *self == PEAKDETMODE_A::AUTOCMD
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == PEAKDETMODE_A::CMD
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PEAKDETMODE_A::MANUAL
    }
}
#[doc = "Field `PEAKDETMODE` writer - HFXO Automatic Peak Detection Mode"]
pub type PEAKDETMODE_W<'a> =
    crate::FieldWriterSafe<'a, u32, HFXOCTRL_SPEC, u8, PEAKDETMODE_A, 2, 4>;
impl<'a> PEAKDETMODE_W<'a> {
    #[doc = "Automatic control of HFXO peak detection sequence. Only performs peak detection on initial HFXO startup. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn oncecmd(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::ONCECMD)
    }
    #[doc = "Automatic control of HFXO peak detection sequence. CMU_CMD HFXOPEAKDETSTART allowed to be used after HFXORDY=1."]
    #[inline(always)]
    pub fn autocmd(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::AUTOCMD)
    }
    #[doc = "CMU_CMD HFXOPEAKDETSTART can be used to trigger the peak detection sequence after HFXORDY=1."]
    #[inline(always)]
    pub fn cmd(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::CMD)
    }
    #[doc = "CMU_HFXOSTEADYSTATECTRL IBTRIMXOCORE and PEAKDETEN are under full software control and are allowed to be changed once HFXO is ready."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PEAKDETMODE_A::MANUAL)
    }
}
#[doc = "HFXO Low Frequency Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFTIMEOUT_A {
    #[doc = "0: Timeout period of 0 cycles (disabled)"]
    _0CYCLES = 0,
    #[doc = "1: Timeout period of 2 cycles"]
    _2CYCLES = 1,
    #[doc = "2: Timeout period of 4 cycles"]
    _4CYCLES = 2,
    #[doc = "3: Timeout period of 16 cycles"]
    _16CYCLES = 3,
    #[doc = "4: Timeout period of 32 cycles"]
    _32CYCLES = 4,
    #[doc = "5: Timeout period of 64 cycles"]
    _64CYCLES = 5,
    #[doc = "6: Timeout period of 1024 cycles"]
    _1KCYCLES = 6,
    #[doc = "7: Timeout period of 4096 cycles"]
    _4KCYCLES = 7,
}
impl From<LFTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFTIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFTIMEOUT` reader - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_R = crate::FieldReader<u8, LFTIMEOUT_A>;
impl LFTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFTIMEOUT_A {
        match self.bits {
            0 => LFTIMEOUT_A::_0CYCLES,
            1 => LFTIMEOUT_A::_2CYCLES,
            2 => LFTIMEOUT_A::_4CYCLES,
            3 => LFTIMEOUT_A::_16CYCLES,
            4 => LFTIMEOUT_A::_32CYCLES,
            5 => LFTIMEOUT_A::_64CYCLES,
            6 => LFTIMEOUT_A::_1KCYCLES,
            7 => LFTIMEOUT_A::_4KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0CYCLES`"]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_0CYCLES
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == LFTIMEOUT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_4KCYCLES`"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == LFTIMEOUT_A::_4KCYCLES
    }
}
#[doc = "Field `LFTIMEOUT` writer - HFXO Low Frequency Timeout"]
pub type LFTIMEOUT_W<'a> = crate::FieldWriterSafe<'a, u32, HFXOCTRL_SPEC, u8, LFTIMEOUT_A, 3, 24>;
impl<'a> LFTIMEOUT_W<'a> {
    #[doc = "Timeout period of 0 cycles (disabled)"]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_0CYCLES)
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_32CYCLES)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_64CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut W {
        self.variant(LFTIMEOUT_A::_4KCYCLES)
    }
}
#[doc = "Field `AUTOSTARTEM0EM1` reader - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTARTEM0EM1` writer - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTEM0EM1_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 28>;
#[doc = "Field `AUTOSTARTSELEM0EM1` reader - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTARTSELEM0EM1` writer - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
pub type AUTOSTARTSELEM0EM1_W<'a> = crate::BitWriter<'a, u32, HFXOCTRL_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&self) -> PEAKDETMODE_R {
        PEAKDETMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&self) -> LFTIMEOUT_R {
        LFTIMEOUT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&self) -> AUTOSTARTEM0EM1_R {
        AUTOSTARTEM0EM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&self) -> AUTOSTARTSELEM0EM1_R {
        AUTOSTARTSELEM0EM1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - HFXO Automatic Peak Detection Mode"]
    #[inline(always)]
    pub fn peakdetmode(&mut self) -> PEAKDETMODE_W {
        PEAKDETMODE_W::new(self)
    }
    #[doc = "Bits 24:26 - HFXO Low Frequency Timeout"]
    #[inline(always)]
    pub fn lftimeout(&mut self) -> LFTIMEOUT_W {
        LFTIMEOUT_W::new(self)
    }
    #[doc = "Bit 28 - Automatically Start of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartem0em1(&mut self) -> AUTOSTARTEM0EM1_W {
        AUTOSTARTEM0EM1_W::new(self)
    }
    #[doc = "Bit 29 - Automatically Start and Select of HFXO Upon EM0/EM1 Entry From EM2/EM3"]
    #[inline(always)]
    pub fn autostartselem0em1(&mut self) -> AUTOSTARTSELEM0EM1_W {
        AUTOSTARTSELEM0EM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxoctrl](index.html) module"]
pub struct HFXOCTRL_SPEC;
impl crate::RegisterSpec for HFXOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxoctrl::R](R) reader structure"]
impl crate::Readable for HFXOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxoctrl::W](W) writer structure"]
impl crate::Writable for HFXOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOCTRL to value 0"]
impl crate::Resettable for HFXOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
