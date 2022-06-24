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
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    ULFRCO = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    LFRCO = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    LFXO = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    HFXO = 6,
    #[doc = "7: HFEXPCLK"]
    HFEXPCLK = 7,
    #[doc = "9: ULFRCO (qualified)"]
    ULFRCOQ = 9,
    #[doc = "10: LFRCO (qualified)"]
    LFRCOQ = 10,
    #[doc = "11: LFXO (qualified)"]
    LFXOQ = 11,
    #[doc = "12: HFRCO (qualified)"]
    HFRCOQ = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    AUXHFRCOQ = 13,
    #[doc = "14: HFXO (qualified)"]
    HFXOQ = 14,
    #[doc = "15: HFSRCCLK"]
    HFSRCCLK = 15,
}
impl From<CLKOUTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type CLKOUTSEL0_R = crate::FieldReader<u8, CLKOUTSEL0_A>;
impl CLKOUTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL0_A> {
        match self.bits {
            0 => Some(CLKOUTSEL0_A::DISABLED),
            1 => Some(CLKOUTSEL0_A::ULFRCO),
            2 => Some(CLKOUTSEL0_A::LFRCO),
            3 => Some(CLKOUTSEL0_A::LFXO),
            6 => Some(CLKOUTSEL0_A::HFXO),
            7 => Some(CLKOUTSEL0_A::HFEXPCLK),
            9 => Some(CLKOUTSEL0_A::ULFRCOQ),
            10 => Some(CLKOUTSEL0_A::LFRCOQ),
            11 => Some(CLKOUTSEL0_A::LFXOQ),
            12 => Some(CLKOUTSEL0_A::HFRCOQ),
            13 => Some(CLKOUTSEL0_A::AUXHFRCOQ),
            14 => Some(CLKOUTSEL0_A::HFXOQ),
            15 => Some(CLKOUTSEL0_A::HFSRCCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFSRCCLK
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type CLKOUTSEL0_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLKOUTSEL0_A, 4, 0>;
impl<'a> CLKOUTSEL0_W<'a> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFSRCCLK)
    }
}
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    ULFRCO = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    LFRCO = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    LFXO = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    HFXO = 6,
    #[doc = "7: HFEXPCLK"]
    HFEXPCLK = 7,
    #[doc = "9: ULFRCO (qualified)"]
    ULFRCOQ = 9,
    #[doc = "10: LFRCO (qualified)"]
    LFRCOQ = 10,
    #[doc = "11: LFXO (qualified)"]
    LFXOQ = 11,
    #[doc = "12: HFRCO (qualified)"]
    HFRCOQ = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    AUXHFRCOQ = 13,
    #[doc = "14: HFXO (qualified)"]
    HFXOQ = 14,
    #[doc = "15: HFSRCCLK"]
    HFSRCCLK = 15,
}
impl From<CLKOUTSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type CLKOUTSEL1_R = crate::FieldReader<u8, CLKOUTSEL1_A>;
impl CLKOUTSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL1_A> {
        match self.bits {
            0 => Some(CLKOUTSEL1_A::DISABLED),
            1 => Some(CLKOUTSEL1_A::ULFRCO),
            2 => Some(CLKOUTSEL1_A::LFRCO),
            3 => Some(CLKOUTSEL1_A::LFXO),
            6 => Some(CLKOUTSEL1_A::HFXO),
            7 => Some(CLKOUTSEL1_A::HFEXPCLK),
            9 => Some(CLKOUTSEL1_A::ULFRCOQ),
            10 => Some(CLKOUTSEL1_A::LFRCOQ),
            11 => Some(CLKOUTSEL1_A::LFXOQ),
            12 => Some(CLKOUTSEL1_A::HFRCOQ),
            13 => Some(CLKOUTSEL1_A::AUXHFRCOQ),
            14 => Some(CLKOUTSEL1_A::HFXOQ),
            15 => Some(CLKOUTSEL1_A::HFSRCCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFSRCCLK
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type CLKOUTSEL1_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLKOUTSEL1_A, 4, 5>;
impl<'a> CLKOUTSEL1_W<'a> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFSRCCLK)
    }
}
#[doc = "Clock Output Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    ULFRCO = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    LFRCO = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    LFXO = 3,
    #[doc = "5: HFXO divided by two (qualified)"]
    HFXODIV2Q = 5,
    #[doc = "6: HFXO (directly from oscillator)"]
    HFXO = 6,
    #[doc = "7: HFEXPCLK"]
    HFEXPCLK = 7,
    #[doc = "9: ULFRCO (qualified)"]
    ULFRCOQ = 9,
    #[doc = "10: LFRCO (qualified)"]
    LFRCOQ = 10,
    #[doc = "11: LFXO (qualified)"]
    LFXOQ = 11,
    #[doc = "12: HFRCO (qualified)"]
    HFRCOQ = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    AUXHFRCOQ = 13,
    #[doc = "14: HFXO (qualified)"]
    HFXOQ = 14,
    #[doc = "15: HFSRCCLK"]
    HFSRCCLK = 15,
}
impl From<CLKOUTSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL2` reader - Clock Output Select 2"]
pub type CLKOUTSEL2_R = crate::FieldReader<u8, CLKOUTSEL2_A>;
impl CLKOUTSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL2_A> {
        match self.bits {
            0 => Some(CLKOUTSEL2_A::DISABLED),
            1 => Some(CLKOUTSEL2_A::ULFRCO),
            2 => Some(CLKOUTSEL2_A::LFRCO),
            3 => Some(CLKOUTSEL2_A::LFXO),
            5 => Some(CLKOUTSEL2_A::HFXODIV2Q),
            6 => Some(CLKOUTSEL2_A::HFXO),
            7 => Some(CLKOUTSEL2_A::HFEXPCLK),
            9 => Some(CLKOUTSEL2_A::ULFRCOQ),
            10 => Some(CLKOUTSEL2_A::LFRCOQ),
            11 => Some(CLKOUTSEL2_A::LFXOQ),
            12 => Some(CLKOUTSEL2_A::HFRCOQ),
            13 => Some(CLKOUTSEL2_A::AUXHFRCOQ),
            14 => Some(CLKOUTSEL2_A::HFXOQ),
            15 => Some(CLKOUTSEL2_A::HFSRCCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXODIV2Q`"]
    #[inline(always)]
    pub fn is_hfxodiv2q(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXODIV2Q
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFSRCCLK
    }
}
#[doc = "Field `CLKOUTSEL2` writer - Clock Output Select 2"]
pub type CLKOUTSEL2_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLKOUTSEL2_A, 4, 10>;
impl<'a> CLKOUTSEL2_W<'a> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFXO)
    }
    #[doc = "HFXO divided by two (qualified)"]
    #[inline(always)]
    pub fn hfxodiv2q(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXODIV2Q)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFSRCCLK)
    }
}
#[doc = "Field `WSHFLE` reader - Wait State for High-Frequency LE Interface"]
pub type WSHFLE_R = crate::BitReader<bool>;
#[doc = "Field `WSHFLE` writer - Wait State for High-Frequency LE Interface"]
pub type WSHFLE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HFPERCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HFPERCLKEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 20>;
impl R {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&self) -> CLKOUTSEL2_R {
        CLKOUTSEL2_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&self) -> WSHFLE_R {
        WSHFLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W {
        CLKOUTSEL0_W::new(self)
    }
    #[doc = "Bits 5:8 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W {
        CLKOUTSEL1_W::new(self)
    }
    #[doc = "Bits 10:13 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&mut self) -> CLKOUTSEL2_W {
        CLKOUTSEL2_W::new(self)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&mut self) -> WSHFLE_W {
        WSHFLE_W::new(self)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W {
        HFPERCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x0010_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
