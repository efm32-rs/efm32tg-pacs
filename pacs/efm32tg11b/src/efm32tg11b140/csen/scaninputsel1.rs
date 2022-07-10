#[doc = "Register `SCANINPUTSEL1` reader"]
pub struct R(crate::R<SCANINPUTSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANINPUTSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANINPUTSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANINPUTSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANINPUTSEL1` writer"]
pub struct W(crate::W<SCANINPUTSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANINPUTSEL1_SPEC>;
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
impl From<crate::W<SCANINPUTSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANINPUTSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CSEN_INPUT32-39 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT32TO39SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT32TO39SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT32TO39SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT32TO39SEL` reader - CSEN_INPUT32-39 Select"]
pub type INPUT32TO39SEL_R = crate::FieldReader<u8, INPUT32TO39SEL_A>;
impl INPUT32TO39SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT32TO39SEL_A> {
        match self.bits {
            4 => Some(INPUT32TO39SEL_A::APORT1CH0TO7),
            5 => Some(INPUT32TO39SEL_A::APORT1CH8TO15),
            6 => Some(INPUT32TO39SEL_A::APORT1CH16TO23),
            7 => Some(INPUT32TO39SEL_A::APORT1CH24TO31),
            12 => Some(INPUT32TO39SEL_A::APORT3CH0TO7),
            13 => Some(INPUT32TO39SEL_A::APORT3CH8TO15),
            14 => Some(INPUT32TO39SEL_A::APORT3CH16TO23),
            15 => Some(INPUT32TO39SEL_A::APORT3CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH24TO31
    }
}
#[doc = "Field `INPUT32TO39SEL` writer - CSEN_INPUT32-39 Select"]
pub type INPUT32TO39SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL1_SPEC, u8, INPUT32TO39SEL_A, 4, 0>;
impl<'a> INPUT32TO39SEL_W<'a> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH24TO31)
    }
}
#[doc = "CSEN_INPUT40-47 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT40TO47SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT40TO47SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT40TO47SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT40TO47SEL` reader - CSEN_INPUT40-47 Select"]
pub type INPUT40TO47SEL_R = crate::FieldReader<u8, INPUT40TO47SEL_A>;
impl INPUT40TO47SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT40TO47SEL_A> {
        match self.bits {
            4 => Some(INPUT40TO47SEL_A::APORT1CH0TO7),
            5 => Some(INPUT40TO47SEL_A::APORT1CH8TO15),
            6 => Some(INPUT40TO47SEL_A::APORT1CH16TO23),
            7 => Some(INPUT40TO47SEL_A::APORT1CH24TO31),
            12 => Some(INPUT40TO47SEL_A::APORT3CH0TO7),
            13 => Some(INPUT40TO47SEL_A::APORT3CH8TO15),
            14 => Some(INPUT40TO47SEL_A::APORT3CH16TO23),
            15 => Some(INPUT40TO47SEL_A::APORT3CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH24TO31
    }
}
#[doc = "Field `INPUT40TO47SEL` writer - CSEN_INPUT40-47 Select"]
pub type INPUT40TO47SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL1_SPEC, u8, INPUT40TO47SEL_A, 4, 8>;
impl<'a> INPUT40TO47SEL_W<'a> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH24TO31)
    }
}
#[doc = "CSEN_INPUT48-55 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT48TO55SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT48TO55SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT48TO55SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT48TO55SEL` reader - CSEN_INPUT48-55 Select"]
pub type INPUT48TO55SEL_R = crate::FieldReader<u8, INPUT48TO55SEL_A>;
impl INPUT48TO55SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT48TO55SEL_A> {
        match self.bits {
            4 => Some(INPUT48TO55SEL_A::APORT1CH0TO7),
            5 => Some(INPUT48TO55SEL_A::APORT1CH8TO15),
            6 => Some(INPUT48TO55SEL_A::APORT1CH16TO23),
            7 => Some(INPUT48TO55SEL_A::APORT1CH24TO31),
            12 => Some(INPUT48TO55SEL_A::APORT3CH0TO7),
            13 => Some(INPUT48TO55SEL_A::APORT3CH8TO15),
            14 => Some(INPUT48TO55SEL_A::APORT3CH16TO23),
            15 => Some(INPUT48TO55SEL_A::APORT3CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH24TO31
    }
}
#[doc = "Field `INPUT48TO55SEL` writer - CSEN_INPUT48-55 Select"]
pub type INPUT48TO55SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL1_SPEC, u8, INPUT48TO55SEL_A, 4, 16>;
impl<'a> INPUT48TO55SEL_W<'a> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH24TO31)
    }
}
#[doc = "CSEN_INPUT56-63 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT56TO63SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT56TO63SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT56TO63SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT56TO63SEL` reader - CSEN_INPUT56-63 Select"]
pub type INPUT56TO63SEL_R = crate::FieldReader<u8, INPUT56TO63SEL_A>;
impl INPUT56TO63SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT56TO63SEL_A> {
        match self.bits {
            4 => Some(INPUT56TO63SEL_A::APORT1CH0TO7),
            5 => Some(INPUT56TO63SEL_A::APORT1CH8TO15),
            6 => Some(INPUT56TO63SEL_A::APORT1CH16TO23),
            7 => Some(INPUT56TO63SEL_A::APORT1CH24TO31),
            12 => Some(INPUT56TO63SEL_A::APORT3CH0TO7),
            13 => Some(INPUT56TO63SEL_A::APORT3CH8TO15),
            14 => Some(INPUT56TO63SEL_A::APORT3CH16TO23),
            15 => Some(INPUT56TO63SEL_A::APORT3CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH24TO31
    }
}
#[doc = "Field `INPUT56TO63SEL` writer - CSEN_INPUT56-63 Select"]
pub type INPUT56TO63SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL1_SPEC, u8, INPUT56TO63SEL_A, 4, 24>;
impl<'a> INPUT56TO63SEL_W<'a> {
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH24TO31)
    }
}
impl R {
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline(always)]
    pub fn input32to39sel(&self) -> INPUT32TO39SEL_R {
        INPUT32TO39SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline(always)]
    pub fn input40to47sel(&self) -> INPUT40TO47SEL_R {
        INPUT40TO47SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline(always)]
    pub fn input48to55sel(&self) -> INPUT48TO55SEL_R {
        INPUT48TO55SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline(always)]
    pub fn input56to63sel(&self) -> INPUT56TO63SEL_R {
        INPUT56TO63SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline(always)]
    pub fn input32to39sel(&mut self) -> INPUT32TO39SEL_W {
        INPUT32TO39SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline(always)]
    pub fn input40to47sel(&mut self) -> INPUT40TO47SEL_W {
        INPUT40TO47SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline(always)]
    pub fn input48to55sel(&mut self) -> INPUT48TO55SEL_W {
        INPUT48TO55SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline(always)]
    pub fn input56to63sel(&mut self) -> INPUT56TO63SEL_W {
        INPUT56TO63SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Input Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scaninputsel1](index.html) module"]
pub struct SCANINPUTSEL1_SPEC;
impl crate::RegisterSpec for SCANINPUTSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scaninputsel1::R](R) reader structure"]
impl crate::Readable for SCANINPUTSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scaninputsel1::W](W) writer structure"]
impl crate::Writable for SCANINPUTSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCANINPUTSEL1 to value 0"]
impl crate::Resettable for SCANINPUTSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
