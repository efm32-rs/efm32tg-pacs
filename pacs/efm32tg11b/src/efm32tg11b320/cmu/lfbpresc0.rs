#[doc = "Register `LFBPRESC0` reader"]
pub struct R(crate::R<LFBPRESC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFBPRESC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFBPRESC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFBPRESC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFBPRESC0` writer"]
pub struct W(crate::W<LFBPRESC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFBPRESC0_SPEC>;
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
impl From<crate::W<LFBPRESC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFBPRESC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTICK` reader - Prescaler"]
pub type SYSTICK_R = crate::FieldReader<u8, SYSTICK_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSTICK_A {
    #[doc = "0: LFBCLKSYSTICK = LFBCLK"]
    DIV1 = 0,
}
impl From<SYSTICK_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSTICK_A) -> Self {
        variant as _
    }
}
impl SYSTICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSTICK_A> {
        match self.bits {
            0 => Some(SYSTICK_A::DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYSTICK_A::DIV1
    }
}
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Prescaler"]
pub type LEUART0_R = crate::FieldReader<u8, LEUART0_A>;
#[doc = "Low Energy UART 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEUART0_A {
    #[doc = "0: LFBCLKLEUART0 = LFBCLK"]
    DIV1 = 0,
    #[doc = "1: LFBCLKLEUART0 = LFBCLK/2"]
    DIV2 = 1,
    #[doc = "2: LFBCLKLEUART0 = LFBCLK/4"]
    DIV4 = 2,
    #[doc = "3: LFBCLKLEUART0 = LFBCLK/8"]
    DIV8 = 3,
}
impl From<LEUART0_A> for u8 {
    #[inline(always)]
    fn from(variant: LEUART0_A) -> Self {
        variant as _
    }
}
impl LEUART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEUART0_A {
        match self.bits {
            0 => LEUART0_A::DIV1,
            1 => LEUART0_A::DIV2,
            2 => LEUART0_A::DIV4,
            3 => LEUART0_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART0_A::DIV8
    }
}
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Prescaler"]
pub type LEUART0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LFBPRESC0_SPEC, u8, LEUART0_A, 2, O>;
impl<'a, const O: u8> LEUART0_W<'a, O> {
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV8)
    }
}
#[doc = "Field `CSEN` reader - Capacitive touch sense module Prescaler"]
pub type CSEN_R = crate::FieldReader<u8, CSEN_A>;
#[doc = "Capacitive touch sense module Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEN_A {
    #[doc = "0: LFBCLKCSEN = LFBCLK/16"]
    DIV16 = 0,
    #[doc = "1: LFBCLKCSEN = LFBCLK/32"]
    DIV32 = 1,
    #[doc = "2: LFBCLKCSEN = LFBCLK/64"]
    DIV64 = 2,
    #[doc = "3: LFBCLKCSEN = LFBCLK/128"]
    DIV128 = 3,
}
impl From<CSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEN_A) -> Self {
        variant as _
    }
}
impl CSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEN_A {
        match self.bits {
            0 => CSEN_A::DIV16,
            1 => CSEN_A::DIV32,
            2 => CSEN_A::DIV64,
            3 => CSEN_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CSEN_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CSEN_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CSEN_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CSEN_A::DIV128
    }
}
#[doc = "Field `CSEN` writer - Capacitive touch sense module Prescaler"]
pub type CSEN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LFBPRESC0_SPEC, u8, CSEN_A, 2, O>;
impl<'a, const O: u8> CSEN_W<'a, O> {
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CSEN_A::DIV16)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CSEN_A::DIV32)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CSEN_A::DIV64)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CSEN_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler"]
    #[inline(always)]
    pub fn systick(&self) -> SYSTICK_R {
        SYSTICK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<4> {
        LEUART0_W::new(self)
    }
    #[doc = "Bits 8:9 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<8> {
        CSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfbpresc0](index.html) module"]
pub struct LFBPRESC0_SPEC;
impl crate::RegisterSpec for LFBPRESC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfbpresc0::R](R) reader structure"]
impl crate::Readable for LFBPRESC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfbpresc0::W](W) writer structure"]
impl crate::Writable for LFBPRESC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFBPRESC0 to value 0"]
impl crate::Resettable for LFBPRESC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
