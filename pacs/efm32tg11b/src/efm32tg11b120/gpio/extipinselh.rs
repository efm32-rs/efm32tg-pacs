#[doc = "Register `EXTIPINSELH` reader"]
pub struct R(crate::R<EXTIPINSELH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPINSELH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPINSELH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPINSELH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPINSELH` writer"]
pub struct W(crate::W<EXTIPINSELH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPINSELH_SPEC>;
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
impl From<crate::W<EXTIPINSELH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPINSELH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Interrupt 8 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL8_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL8` reader - External Interrupt 8 Pin Select"]
pub type EXTIPINSEL8_R = crate::FieldReader<u8, EXTIPINSEL8_A>;
impl EXTIPINSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL8_A {
        match self.bits {
            0 => EXTIPINSEL8_A::PIN8,
            1 => EXTIPINSEL8_A::PIN9,
            2 => EXTIPINSEL8_A::PIN10,
            3 => EXTIPINSEL8_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN11
    }
}
#[doc = "Field `EXTIPINSEL8` writer - External Interrupt 8 Pin Select"]
pub type EXTIPINSEL8_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL8_A, 2, 0>;
impl<'a> EXTIPINSEL8_W<'a> {
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN11)
    }
}
#[doc = "External Interrupt 9 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL9_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL9` reader - External Interrupt 9 Pin Select"]
pub type EXTIPINSEL9_R = crate::FieldReader<u8, EXTIPINSEL9_A>;
impl EXTIPINSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL9_A {
        match self.bits {
            0 => EXTIPINSEL9_A::PIN8,
            1 => EXTIPINSEL9_A::PIN9,
            2 => EXTIPINSEL9_A::PIN10,
            3 => EXTIPINSEL9_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN11
    }
}
#[doc = "Field `EXTIPINSEL9` writer - External Interrupt 9 Pin Select"]
pub type EXTIPINSEL9_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL9_A, 2, 4>;
impl<'a> EXTIPINSEL9_W<'a> {
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN11)
    }
}
#[doc = "External Interrupt 10 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL10_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL10` reader - External Interrupt 10 Pin Select"]
pub type EXTIPINSEL10_R = crate::FieldReader<u8, EXTIPINSEL10_A>;
impl EXTIPINSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL10_A {
        match self.bits {
            0 => EXTIPINSEL10_A::PIN8,
            1 => EXTIPINSEL10_A::PIN9,
            2 => EXTIPINSEL10_A::PIN10,
            3 => EXTIPINSEL10_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN11
    }
}
#[doc = "Field `EXTIPINSEL10` writer - External Interrupt 10 Pin Select"]
pub type EXTIPINSEL10_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL10_A, 2, 8>;
impl<'a> EXTIPINSEL10_W<'a> {
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN11)
    }
}
#[doc = "External Interrupt 11 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL11_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL11` reader - External Interrupt 11 Pin Select"]
pub type EXTIPINSEL11_R = crate::FieldReader<u8, EXTIPINSEL11_A>;
impl EXTIPINSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL11_A {
        match self.bits {
            0 => EXTIPINSEL11_A::PIN8,
            1 => EXTIPINSEL11_A::PIN9,
            2 => EXTIPINSEL11_A::PIN10,
            3 => EXTIPINSEL11_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN11
    }
}
#[doc = "Field `EXTIPINSEL11` writer - External Interrupt 11 Pin Select"]
pub type EXTIPINSEL11_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL11_A, 2, 12>;
impl<'a> EXTIPINSEL11_W<'a> {
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN11)
    }
}
#[doc = "External Interrupt 12 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL12_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL12` reader - External Interrupt 12 Pin Select"]
pub type EXTIPINSEL12_R = crate::FieldReader<u8, EXTIPINSEL12_A>;
impl EXTIPINSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL12_A {
        match self.bits {
            0 => EXTIPINSEL12_A::PIN12,
            1 => EXTIPINSEL12_A::PIN13,
            2 => EXTIPINSEL12_A::PIN14,
            3 => EXTIPINSEL12_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN15
    }
}
#[doc = "Field `EXTIPINSEL12` writer - External Interrupt 12 Pin Select"]
pub type EXTIPINSEL12_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL12_A, 2, 16>;
impl<'a> EXTIPINSEL12_W<'a> {
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN15)
    }
}
#[doc = "External Interrupt 13 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL13_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL13` reader - External Interrupt 13 Pin Select"]
pub type EXTIPINSEL13_R = crate::FieldReader<u8, EXTIPINSEL13_A>;
impl EXTIPINSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL13_A {
        match self.bits {
            0 => EXTIPINSEL13_A::PIN12,
            1 => EXTIPINSEL13_A::PIN13,
            2 => EXTIPINSEL13_A::PIN14,
            3 => EXTIPINSEL13_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN15
    }
}
#[doc = "Field `EXTIPINSEL13` writer - External Interrupt 13 Pin Select"]
pub type EXTIPINSEL13_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL13_A, 2, 20>;
impl<'a> EXTIPINSEL13_W<'a> {
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN15)
    }
}
#[doc = "External Interrupt 14 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL14_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL14` reader - External Interrupt 14 Pin Select"]
pub type EXTIPINSEL14_R = crate::FieldReader<u8, EXTIPINSEL14_A>;
impl EXTIPINSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL14_A {
        match self.bits {
            0 => EXTIPINSEL14_A::PIN12,
            1 => EXTIPINSEL14_A::PIN13,
            2 => EXTIPINSEL14_A::PIN14,
            3 => EXTIPINSEL14_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN15
    }
}
#[doc = "Field `EXTIPINSEL14` writer - External Interrupt 14 Pin Select"]
pub type EXTIPINSEL14_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL14_A, 2, 24>;
impl<'a> EXTIPINSEL14_W<'a> {
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN15)
    }
}
#[doc = "External Interrupt 15 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL15_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPINSEL15` reader - External Interrupt 15 Pin Select"]
pub type EXTIPINSEL15_R = crate::FieldReader<u8, EXTIPINSEL15_A>;
impl EXTIPINSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL15_A {
        match self.bits {
            0 => EXTIPINSEL15_A::PIN12,
            1 => EXTIPINSEL15_A::PIN13,
            2 => EXTIPINSEL15_A::PIN14,
            3 => EXTIPINSEL15_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN15
    }
}
#[doc = "Field `EXTIPINSEL15` writer - External Interrupt 15 Pin Select"]
pub type EXTIPINSEL15_W<'a> =
    crate::FieldWriterSafe<'a, u32, EXTIPINSELH_SPEC, u8, EXTIPINSEL15_A, 2, 28>;
impl<'a> EXTIPINSEL15_W<'a> {
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN15)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&self) -> EXTIPINSEL8_R {
        EXTIPINSEL8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&self) -> EXTIPINSEL9_R {
        EXTIPINSEL9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&self) -> EXTIPINSEL10_R {
        EXTIPINSEL10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&self) -> EXTIPINSEL11_R {
        EXTIPINSEL11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&self) -> EXTIPINSEL12_R {
        EXTIPINSEL12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&self) -> EXTIPINSEL13_R {
        EXTIPINSEL13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&self) -> EXTIPINSEL14_R {
        EXTIPINSEL14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&self) -> EXTIPINSEL15_R {
        EXTIPINSEL15_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&mut self) -> EXTIPINSEL8_W {
        EXTIPINSEL8_W::new(self)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&mut self) -> EXTIPINSEL9_W {
        EXTIPINSEL9_W::new(self)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&mut self) -> EXTIPINSEL10_W {
        EXTIPINSEL10_W::new(self)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&mut self) -> EXTIPINSEL11_W {
        EXTIPINSEL11_W::new(self)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&mut self) -> EXTIPINSEL12_W {
        EXTIPINSEL12_W::new(self)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&mut self) -> EXTIPINSEL13_W {
        EXTIPINSEL13_W::new(self)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&mut self) -> EXTIPINSEL14_W {
        EXTIPINSEL14_W::new(self)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&mut self) -> EXTIPINSEL15_W {
        EXTIPINSEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Pin Select High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipinselh](index.html) module"]
pub struct EXTIPINSELH_SPEC;
impl crate::RegisterSpec for EXTIPINSELH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipinselh::R](R) reader structure"]
impl crate::Readable for EXTIPINSELH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipinselh::W](W) writer structure"]
impl crate::Writable for EXTIPINSELH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTIPINSELH to value 0x3210_3210"]
impl crate::Resettable for EXTIPINSELH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3210_3210
    }
}
