#[doc = "Register `LFRCOCTRL` reader"]
pub struct R(crate::R<LFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFRCOCTRL` writer"]
pub struct W(crate::W<LFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFRCOCTRL_SPEC>;
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
impl From<crate::W<LFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - LFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TUNING` writer - LFRCO Tuning Value"]
pub type TUNING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFRCOCTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `ENVREF` reader - Enable Duty Cycling of Vref"]
pub type ENVREF_R = crate::BitReader<bool>;
#[doc = "Field `ENVREF` writer - Enable Duty Cycling of Vref"]
pub type ENVREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFRCOCTRL_SPEC, bool, O>;
#[doc = "Field `ENCHOP` reader - Enable Comparator Chopping"]
pub type ENCHOP_R = crate::BitReader<bool>;
#[doc = "Field `ENCHOP` writer - Enable Comparator Chopping"]
pub type ENCHOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFRCOCTRL_SPEC, bool, O>;
#[doc = "Field `ENDEM` reader - Enable Dynamic Element Matching"]
pub type ENDEM_R = crate::BitReader<bool>;
#[doc = "Field `ENDEM` writer - Enable Dynamic Element Matching"]
pub type ENDEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFRCOCTRL_SPEC, bool, O>;
#[doc = "Field `VREFUPDATE` reader - Control Vref Update Rate"]
pub type VREFUPDATE_R = crate::FieldReader<u8, VREFUPDATE_A>;
#[doc = "Control Vref Update Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFUPDATE_A {
    #[doc = "0: 32 clocks."]
    _32CYCLES = 0,
    #[doc = "1: 64 clocks."]
    _64CYCLES = 1,
    #[doc = "2: 128 clocks."]
    _128CYCLES = 2,
    #[doc = "3: 256 clocks."]
    _256CYCLES = 3,
}
impl From<VREFUPDATE_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFUPDATE_A) -> Self {
        variant as _
    }
}
impl VREFUPDATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFUPDATE_A {
        match self.bits {
            0 => VREFUPDATE_A::_32CYCLES,
            1 => VREFUPDATE_A::_64CYCLES,
            2 => VREFUPDATE_A::_128CYCLES,
            3 => VREFUPDATE_A::_256CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == VREFUPDATE_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == VREFUPDATE_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == VREFUPDATE_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == VREFUPDATE_A::_256CYCLES
    }
}
#[doc = "Field `VREFUPDATE` writer - Control Vref Update Rate"]
pub type VREFUPDATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LFRCOCTRL_SPEC, u8, VREFUPDATE_A, 2, O>;
impl<'a, const O: u8> VREFUPDATE_W<'a, O> {
    #[doc = "32 clocks."]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_32CYCLES)
    }
    #[doc = "64 clocks."]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_64CYCLES)
    }
    #[doc = "128 clocks."]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_128CYCLES)
    }
    #[doc = "256 clocks."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(VREFUPDATE_A::_256CYCLES)
    }
}
#[doc = "Field `TIMEOUT` reader - LFRCO Timeout"]
pub type TIMEOUT_R = crate::FieldReader<u8, TIMEOUT_A>;
#[doc = "LFRCO Timeout\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Timeout period of 2 cycles"]
    _2CYCLES = 0,
    #[doc = "1: Timeout period of 16 cycles"]
    _16CYCLES = 1,
    #[doc = "2: Timeout period of 32 cycles"]
    _32CYCLES = 2,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_A> {
        match self.bits {
            0 => Some(TIMEOUT_A::_2CYCLES),
            1 => Some(TIMEOUT_A::_16CYCLES),
            2 => Some(TIMEOUT_A::_32CYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == TIMEOUT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == TIMEOUT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == TIMEOUT_A::_32CYCLES
    }
}
#[doc = "Field `TIMEOUT` writer - LFRCO Timeout"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LFRCOCTRL_SPEC, u8, TIMEOUT_A, 2, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_2CYCLES)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_16CYCLES)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_32CYCLES)
    }
}
#[doc = "Field `GMCCURTUNE` reader - Tuning of Gmc Current"]
pub type GMCCURTUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GMCCURTUNE` writer - Tuning of Gmc Current"]
pub type GMCCURTUNE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFRCOCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    pub fn envref(&self) -> ENVREF_R {
        ENVREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    pub fn enchop(&self) -> ENCHOP_R {
        ENCHOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    pub fn endem(&self) -> ENDEM_R {
        ENDEM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Control Vref Update Rate"]
    #[inline(always)]
    pub fn vrefupdate(&self) -> VREFUPDATE_R {
        VREFUPDATE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    pub fn gmccurtune(&self) -> GMCCURTUNE_R {
        GMCCURTUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - LFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<0> {
        TUNING_W::new(self)
    }
    #[doc = "Bit 16 - Enable Duty Cycling of Vref"]
    #[inline(always)]
    #[must_use]
    pub fn envref(&mut self) -> ENVREF_W<16> {
        ENVREF_W::new(self)
    }
    #[doc = "Bit 17 - Enable Comparator Chopping"]
    #[inline(always)]
    #[must_use]
    pub fn enchop(&mut self) -> ENCHOP_W<17> {
        ENCHOP_W::new(self)
    }
    #[doc = "Bit 18 - Enable Dynamic Element Matching"]
    #[inline(always)]
    #[must_use]
    pub fn endem(&mut self) -> ENDEM_W<18> {
        ENDEM_W::new(self)
    }
    #[doc = "Bits 20:21 - Control Vref Update Rate"]
    #[inline(always)]
    #[must_use]
    pub fn vrefupdate(&mut self) -> VREFUPDATE_W<20> {
        VREFUPDATE_W::new(self)
    }
    #[doc = "Bits 24:25 - LFRCO Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<24> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bits 28:31 - Tuning of Gmc Current"]
    #[inline(always)]
    #[must_use]
    pub fn gmccurtune(&mut self) -> GMCCURTUNE_W<28> {
        GMCCURTUNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfrcoctrl](index.html) module"]
pub struct LFRCOCTRL_SPEC;
impl crate::RegisterSpec for LFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfrcoctrl::R](R) reader structure"]
impl crate::Readable for LFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfrcoctrl::W](W) writer structure"]
impl crate::Writable for LFRCOCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFRCOCTRL to value 0x8106_0100"]
impl crate::Resettable for LFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8106_0100;
}
