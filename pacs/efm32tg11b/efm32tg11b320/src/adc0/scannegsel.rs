#[doc = "Register `SCANNEGSEL` reader"]
pub struct R(crate::R<SCANNEGSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANNEGSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANNEGSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANNEGSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANNEGSEL` writer"]
pub struct W(crate::W<SCANNEGSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANNEGSEL_SPEC>;
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
impl From<crate::W<SCANNEGSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANNEGSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT0NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7 = 3,
}
impl From<INPUT0NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT0NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT0NEGSEL` reader - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
pub type INPUT0NEGSEL_R = crate::FieldReader<u8, INPUT0NEGSEL_A>;
impl INPUT0NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT0NEGSEL_A {
        match self.bits {
            0 => INPUT0NEGSEL_A::INPUT1,
            1 => INPUT0NEGSEL_A::INPUT3,
            2 => INPUT0NEGSEL_A::INPUT5,
            3 => INPUT0NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT7
    }
}
#[doc = "Field `INPUT0NEGSEL` writer - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
pub type INPUT0NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT0NEGSEL_A, 2, 0>;
impl<'a> INPUT0NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT2NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7 = 3,
}
impl From<INPUT2NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT2NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT2NEGSEL` reader - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
pub type INPUT2NEGSEL_R = crate::FieldReader<u8, INPUT2NEGSEL_A>;
impl INPUT2NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT2NEGSEL_A {
        match self.bits {
            0 => INPUT2NEGSEL_A::INPUT1,
            1 => INPUT2NEGSEL_A::INPUT3,
            2 => INPUT2NEGSEL_A::INPUT5,
            3 => INPUT2NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT7
    }
}
#[doc = "Field `INPUT2NEGSEL` writer - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
pub type INPUT2NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT2NEGSEL_A, 2, 2>;
impl<'a> INPUT2NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT4NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7 = 3,
}
impl From<INPUT4NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT4NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT4NEGSEL` reader - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
pub type INPUT4NEGSEL_R = crate::FieldReader<u8, INPUT4NEGSEL_A>;
impl INPUT4NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT4NEGSEL_A {
        match self.bits {
            0 => INPUT4NEGSEL_A::INPUT1,
            1 => INPUT4NEGSEL_A::INPUT3,
            2 => INPUT4NEGSEL_A::INPUT5,
            3 => INPUT4NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT7
    }
}
#[doc = "Field `INPUT4NEGSEL` writer - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
pub type INPUT4NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT4NEGSEL_A, 2, 4>;
impl<'a> INPUT4NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT6NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7 = 3,
}
impl From<INPUT6NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT6NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT6NEGSEL` reader - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
pub type INPUT6NEGSEL_R = crate::FieldReader<u8, INPUT6NEGSEL_A>;
impl INPUT6NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT6NEGSEL_A {
        match self.bits {
            0 => INPUT6NEGSEL_A::INPUT1,
            1 => INPUT6NEGSEL_A::INPUT3,
            2 => INPUT6NEGSEL_A::INPUT5,
            3 => INPUT6NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT7
    }
}
#[doc = "Field `INPUT6NEGSEL` writer - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
pub type INPUT6NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT6NEGSEL_A, 2, 6>;
impl<'a> INPUT6NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT9NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14 = 3,
}
impl From<INPUT9NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT9NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT9NEGSEL` reader - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
pub type INPUT9NEGSEL_R = crate::FieldReader<u8, INPUT9NEGSEL_A>;
impl INPUT9NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT9NEGSEL_A {
        match self.bits {
            0 => INPUT9NEGSEL_A::INPUT8,
            1 => INPUT9NEGSEL_A::INPUT10,
            2 => INPUT9NEGSEL_A::INPUT12,
            3 => INPUT9NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT14
    }
}
#[doc = "Field `INPUT9NEGSEL` writer - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
pub type INPUT9NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT9NEGSEL_A, 2, 8>;
impl<'a> INPUT9NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT14)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT11NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14 = 3,
}
impl From<INPUT11NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT11NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT11NEGSEL` reader - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
pub type INPUT11NEGSEL_R = crate::FieldReader<u8, INPUT11NEGSEL_A>;
impl INPUT11NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT11NEGSEL_A {
        match self.bits {
            0 => INPUT11NEGSEL_A::INPUT8,
            1 => INPUT11NEGSEL_A::INPUT10,
            2 => INPUT11NEGSEL_A::INPUT12,
            3 => INPUT11NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT14
    }
}
#[doc = "Field `INPUT11NEGSEL` writer - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
pub type INPUT11NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT11NEGSEL_A, 2, 10>;
impl<'a> INPUT11NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT14)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT13NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14 = 3,
}
impl From<INPUT13NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT13NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT13NEGSEL` reader - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
pub type INPUT13NEGSEL_R = crate::FieldReader<u8, INPUT13NEGSEL_A>;
impl INPUT13NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT13NEGSEL_A {
        match self.bits {
            0 => INPUT13NEGSEL_A::INPUT8,
            1 => INPUT13NEGSEL_A::INPUT10,
            2 => INPUT13NEGSEL_A::INPUT12,
            3 => INPUT13NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT14
    }
}
#[doc = "Field `INPUT13NEGSEL` writer - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
pub type INPUT13NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT13NEGSEL_A, 2, 12>;
impl<'a> INPUT13NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT14)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT15NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14 = 3,
}
impl From<INPUT15NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT15NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPUT15NEGSEL` reader - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
pub type INPUT15NEGSEL_R = crate::FieldReader<u8, INPUT15NEGSEL_A>;
impl INPUT15NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT15NEGSEL_A {
        match self.bits {
            0 => INPUT15NEGSEL_A::INPUT8,
            1 => INPUT15NEGSEL_A::INPUT10,
            2 => INPUT15NEGSEL_A::INPUT12,
            3 => INPUT15NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT14
    }
}
#[doc = "Field `INPUT15NEGSEL` writer - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
pub type INPUT15NEGSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, SCANNEGSEL_SPEC, u8, INPUT15NEGSEL_A, 2, 14>;
impl<'a> INPUT15NEGSEL_W<'a> {
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT14)
    }
}
impl R {
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input0negsel(&self) -> INPUT0NEGSEL_R {
        INPUT0NEGSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input2negsel(&self) -> INPUT2NEGSEL_R {
        INPUT2NEGSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input4negsel(&self) -> INPUT4NEGSEL_R {
        INPUT4NEGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input6negsel(&self) -> INPUT6NEGSEL_R {
        INPUT6NEGSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input9negsel(&self) -> INPUT9NEGSEL_R {
        INPUT9NEGSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input11negsel(&self) -> INPUT11NEGSEL_R {
        INPUT11NEGSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input13negsel(&self) -> INPUT13NEGSEL_R {
        INPUT13NEGSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input15negsel(&self) -> INPUT15NEGSEL_R {
        INPUT15NEGSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input0negsel(&mut self) -> INPUT0NEGSEL_W {
        INPUT0NEGSEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input2negsel(&mut self) -> INPUT2NEGSEL_W {
        INPUT2NEGSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input4negsel(&mut self) -> INPUT4NEGSEL_W {
        INPUT4NEGSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input6negsel(&mut self) -> INPUT6NEGSEL_W {
        INPUT6NEGSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input9negsel(&mut self) -> INPUT9NEGSEL_W {
        INPUT9NEGSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input11negsel(&mut self) -> INPUT11NEGSEL_W {
        INPUT11NEGSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input13negsel(&mut self) -> INPUT13NEGSEL_W {
        INPUT13NEGSEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input15negsel(&mut self) -> INPUT15NEGSEL_W {
        INPUT15NEGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Negative Input Select Register for Scan\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scannegsel](index.html) module"]
pub struct SCANNEGSEL_SPEC;
impl crate::RegisterSpec for SCANNEGSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scannegsel::R](R) reader structure"]
impl crate::Readable for SCANNEGSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scannegsel::W](W) writer structure"]
impl crate::Writable for SCANNEGSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCANNEGSEL to value 0x39e4"]
impl crate::Resettable for SCANNEGSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x39e4
    }
}
