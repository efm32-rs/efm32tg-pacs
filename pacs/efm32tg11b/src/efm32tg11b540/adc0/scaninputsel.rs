#[doc = "Register `SCANINPUTSEL` reader"]
pub struct R(crate::R<SCANINPUTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANINPUTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANINPUTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANINPUTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANINPUTSEL` writer"]
pub struct W(crate::W<SCANINPUTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANINPUTSEL_SPEC>;
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
impl From<crate::W<SCANINPUTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANINPUTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT0TO7SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT0TO7SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT0TO7SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT0TO7SEL` reader - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
pub type INPUT0TO7SEL_R = crate::FieldReader<u8, INPUT0TO7SEL_A>;
impl INPUT0TO7SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT0TO7SEL_A> {
        match self.bits {
            0 => Some(INPUT0TO7SEL_A::APORT0CH0TO7),
            1 => Some(INPUT0TO7SEL_A::APORT0CH8TO15),
            4 => Some(INPUT0TO7SEL_A::APORT1CH0TO7),
            5 => Some(INPUT0TO7SEL_A::APORT1CH8TO15),
            6 => Some(INPUT0TO7SEL_A::APORT1CH16TO23),
            7 => Some(INPUT0TO7SEL_A::APORT1CH24TO31),
            8 => Some(INPUT0TO7SEL_A::APORT2CH0TO7),
            9 => Some(INPUT0TO7SEL_A::APORT2CH8TO15),
            10 => Some(INPUT0TO7SEL_A::APORT2CH16TO23),
            11 => Some(INPUT0TO7SEL_A::APORT2CH24TO31),
            12 => Some(INPUT0TO7SEL_A::APORT3CH0TO7),
            13 => Some(INPUT0TO7SEL_A::APORT3CH8TO15),
            14 => Some(INPUT0TO7SEL_A::APORT3CH16TO23),
            15 => Some(INPUT0TO7SEL_A::APORT3CH24TO31),
            16 => Some(INPUT0TO7SEL_A::APORT4CH0TO7),
            17 => Some(INPUT0TO7SEL_A::APORT4CH8TO15),
            18 => Some(INPUT0TO7SEL_A::APORT4CH16TO23),
            19 => Some(INPUT0TO7SEL_A::APORT4CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH24TO31
    }
}
#[doc = "Field `INPUT0TO7SEL` writer - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
pub type INPUT0TO7SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL_SPEC, u8, INPUT0TO7SEL_A, 5, 0>;
impl<'a> INPUT0TO7SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH24TO31)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT8TO15SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT8TO15SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT8TO15SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT8TO15SEL` reader - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
pub type INPUT8TO15SEL_R = crate::FieldReader<u8, INPUT8TO15SEL_A>;
impl INPUT8TO15SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT8TO15SEL_A> {
        match self.bits {
            0 => Some(INPUT8TO15SEL_A::APORT0CH0TO7),
            1 => Some(INPUT8TO15SEL_A::APORT0CH8TO15),
            4 => Some(INPUT8TO15SEL_A::APORT1CH0TO7),
            5 => Some(INPUT8TO15SEL_A::APORT1CH8TO15),
            6 => Some(INPUT8TO15SEL_A::APORT1CH16TO23),
            7 => Some(INPUT8TO15SEL_A::APORT1CH24TO31),
            8 => Some(INPUT8TO15SEL_A::APORT2CH0TO7),
            9 => Some(INPUT8TO15SEL_A::APORT2CH8TO15),
            10 => Some(INPUT8TO15SEL_A::APORT2CH16TO23),
            11 => Some(INPUT8TO15SEL_A::APORT2CH24TO31),
            12 => Some(INPUT8TO15SEL_A::APORT3CH0TO7),
            13 => Some(INPUT8TO15SEL_A::APORT3CH8TO15),
            14 => Some(INPUT8TO15SEL_A::APORT3CH16TO23),
            15 => Some(INPUT8TO15SEL_A::APORT3CH24TO31),
            16 => Some(INPUT8TO15SEL_A::APORT4CH0TO7),
            17 => Some(INPUT8TO15SEL_A::APORT4CH8TO15),
            18 => Some(INPUT8TO15SEL_A::APORT4CH16TO23),
            19 => Some(INPUT8TO15SEL_A::APORT4CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH24TO31
    }
}
#[doc = "Field `INPUT8TO15SEL` writer - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
pub type INPUT8TO15SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL_SPEC, u8, INPUT8TO15SEL_A, 5, 8>;
impl<'a> INPUT8TO15SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH24TO31)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT16TO23SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT16TO23SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT16TO23SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT16TO23SEL` reader - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
pub type INPUT16TO23SEL_R = crate::FieldReader<u8, INPUT16TO23SEL_A>;
impl INPUT16TO23SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT16TO23SEL_A> {
        match self.bits {
            0 => Some(INPUT16TO23SEL_A::APORT0CH0TO7),
            1 => Some(INPUT16TO23SEL_A::APORT0CH8TO15),
            4 => Some(INPUT16TO23SEL_A::APORT1CH0TO7),
            5 => Some(INPUT16TO23SEL_A::APORT1CH8TO15),
            6 => Some(INPUT16TO23SEL_A::APORT1CH16TO23),
            7 => Some(INPUT16TO23SEL_A::APORT1CH24TO31),
            8 => Some(INPUT16TO23SEL_A::APORT2CH0TO7),
            9 => Some(INPUT16TO23SEL_A::APORT2CH8TO15),
            10 => Some(INPUT16TO23SEL_A::APORT2CH16TO23),
            11 => Some(INPUT16TO23SEL_A::APORT2CH24TO31),
            12 => Some(INPUT16TO23SEL_A::APORT3CH0TO7),
            13 => Some(INPUT16TO23SEL_A::APORT3CH8TO15),
            14 => Some(INPUT16TO23SEL_A::APORT3CH16TO23),
            15 => Some(INPUT16TO23SEL_A::APORT3CH24TO31),
            16 => Some(INPUT16TO23SEL_A::APORT4CH0TO7),
            17 => Some(INPUT16TO23SEL_A::APORT4CH8TO15),
            18 => Some(INPUT16TO23SEL_A::APORT4CH16TO23),
            19 => Some(INPUT16TO23SEL_A::APORT4CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH24TO31
    }
}
#[doc = "Field `INPUT16TO23SEL` writer - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
pub type INPUT16TO23SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL_SPEC, u8, INPUT16TO23SEL_A, 5, 16>;
impl<'a> INPUT16TO23SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH24TO31)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT24TO31SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT24TO31SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT24TO31SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT24TO31SEL` reader - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
pub type INPUT24TO31SEL_R = crate::FieldReader<u8, INPUT24TO31SEL_A>;
impl INPUT24TO31SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUT24TO31SEL_A> {
        match self.bits {
            0 => Some(INPUT24TO31SEL_A::APORT0CH0TO7),
            1 => Some(INPUT24TO31SEL_A::APORT0CH8TO15),
            4 => Some(INPUT24TO31SEL_A::APORT1CH0TO7),
            5 => Some(INPUT24TO31SEL_A::APORT1CH8TO15),
            6 => Some(INPUT24TO31SEL_A::APORT1CH16TO23),
            7 => Some(INPUT24TO31SEL_A::APORT1CH24TO31),
            8 => Some(INPUT24TO31SEL_A::APORT2CH0TO7),
            9 => Some(INPUT24TO31SEL_A::APORT2CH8TO15),
            10 => Some(INPUT24TO31SEL_A::APORT2CH16TO23),
            11 => Some(INPUT24TO31SEL_A::APORT2CH24TO31),
            12 => Some(INPUT24TO31SEL_A::APORT3CH0TO7),
            13 => Some(INPUT24TO31SEL_A::APORT3CH8TO15),
            14 => Some(INPUT24TO31SEL_A::APORT3CH16TO23),
            15 => Some(INPUT24TO31SEL_A::APORT3CH24TO31),
            16 => Some(INPUT24TO31SEL_A::APORT4CH0TO7),
            17 => Some(INPUT24TO31SEL_A::APORT4CH8TO15),
            18 => Some(INPUT24TO31SEL_A::APORT4CH16TO23),
            19 => Some(INPUT24TO31SEL_A::APORT4CH24TO31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH24TO31
    }
}
#[doc = "Field `INPUT24TO31SEL` writer - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
pub type INPUT24TO31SEL_W<'a> =
    crate::FieldWriter<'a, u32, SCANINPUTSEL_SPEC, u8, INPUT24TO31SEL_A, 5, 24>;
impl<'a> INPUT24TO31SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH24TO31)
    }
}
impl R {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&self) -> INPUT0TO7SEL_R {
        INPUT0TO7SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&self) -> INPUT8TO15SEL_R {
        INPUT8TO15SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&self) -> INPUT16TO23SEL_R {
        INPUT16TO23SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&self) -> INPUT24TO31SEL_R {
        INPUT24TO31SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&mut self) -> INPUT0TO7SEL_W {
        INPUT0TO7SEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&mut self) -> INPUT8TO15SEL_W {
        INPUT8TO15SEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&mut self) -> INPUT16TO23SEL_W {
        INPUT16TO23SEL_W::new(self)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&mut self) -> INPUT24TO31SEL_W {
        INPUT24TO31SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Selection Register for Scan Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scaninputsel](index.html) module"]
pub struct SCANINPUTSEL_SPEC;
impl crate::RegisterSpec for SCANINPUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scaninputsel::R](R) reader structure"]
impl crate::Readable for SCANINPUTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scaninputsel::W](W) writer structure"]
impl crate::Writable for SCANINPUTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCANINPUTSEL to value 0"]
impl crate::Resettable for SCANINPUTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
