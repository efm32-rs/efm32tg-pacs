#[doc = "Register `PRSSEL` reader"]
pub struct R(crate::R<PRSSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSSEL` writer"]
pub struct W(crate::W<PRSSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSSEL_SPEC>;
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
impl From<crate::W<PRSSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRSSTARTSEL` reader - PRS Start Select"]
pub type PRSSTARTSEL_R = crate::FieldReader<u8, PRSSTARTSEL_A>;
#[doc = "PRS Start Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
}
impl From<PRSSTARTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTSEL_A) -> Self {
        variant as _
    }
}
impl PRSSTARTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSTARTSEL_A {
        match self.bits {
            0 => PRSSTARTSEL_A::PRSCH0,
            1 => PRSSTARTSEL_A::PRSCH1,
            2 => PRSSTARTSEL_A::PRSCH2,
            3 => PRSSTARTSEL_A::PRSCH3,
            4 => PRSSTARTSEL_A::PRSCH4,
            5 => PRSSTARTSEL_A::PRSCH5,
            6 => PRSSTARTSEL_A::PRSCH6,
            7 => PRSSTARTSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSSTARTSEL` writer - PRS Start Select"]
pub type PRSSTARTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSSTARTSEL_A, 3, O>;
impl<'a, const O: u8> PRSSTARTSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSTARTSEL_A::PRSCH7)
    }
}
#[doc = "Field `PRSSTOPSEL` reader - PRS Stop Select"]
pub type PRSSTOPSEL_R = crate::FieldReader<u8, PRSSTOPSEL_A>;
#[doc = "PRS Stop Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
}
impl From<PRSSTOPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPSEL_A) -> Self {
        variant as _
    }
}
impl PRSSTOPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSTOPSEL_A {
        match self.bits {
            0 => PRSSTOPSEL_A::PRSCH0,
            1 => PRSSTOPSEL_A::PRSCH1,
            2 => PRSSTOPSEL_A::PRSCH2,
            3 => PRSSTOPSEL_A::PRSCH3,
            4 => PRSSTOPSEL_A::PRSCH4,
            5 => PRSSTOPSEL_A::PRSCH5,
            6 => PRSSTOPSEL_A::PRSCH6,
            7 => PRSSTOPSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSSTOPSEL` writer - PRS Stop Select"]
pub type PRSSTOPSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSSTOPSEL_A, 3, O>;
impl<'a, const O: u8> PRSSTOPSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSTOPSEL_A::PRSCH7)
    }
}
#[doc = "Field `PRSCLEARSEL` reader - PRS Clear Select"]
pub type PRSCLEARSEL_R = crate::FieldReader<u8, PRSCLEARSEL_A>;
#[doc = "PRS Clear Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
}
impl From<PRSCLEARSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARSEL_A) -> Self {
        variant as _
    }
}
impl PRSCLEARSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSCLEARSEL_A {
        match self.bits {
            0 => PRSCLEARSEL_A::PRSCH0,
            1 => PRSCLEARSEL_A::PRSCH1,
            2 => PRSCLEARSEL_A::PRSCH2,
            3 => PRSCLEARSEL_A::PRSCH3,
            4 => PRSCLEARSEL_A::PRSCH4,
            5 => PRSCLEARSEL_A::PRSCH5,
            6 => PRSCLEARSEL_A::PRSCH6,
            7 => PRSCLEARSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSCLEARSEL` writer - PRS Clear Select"]
pub type PRSCLEARSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSCLEARSEL_A, 3, O>;
impl<'a, const O: u8> PRSCLEARSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSCLEARSEL_A::PRSCH7)
    }
}
#[doc = "Field `PRSSTARTMODE` reader - PRS Start Mode"]
pub type PRSSTARTMODE_R = crate::FieldReader<u8, PRSSTARTMODE_A>;
#[doc = "PRS Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTMODE_A {
    #[doc = "0: PRS cannot start the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can start the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can start the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    BOTH = 3,
}
impl From<PRSSTARTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTMODE_A) -> Self {
        variant as _
    }
}
impl PRSSTARTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSTARTMODE_A {
        match self.bits {
            0 => PRSSTARTMODE_A::NONE,
            1 => PRSSTARTMODE_A::RISING,
            2 => PRSSTARTMODE_A::FALLING,
            3 => PRSSTARTMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTARTMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTARTMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTARTMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTARTMODE_A::BOTH
    }
}
#[doc = "Field `PRSSTARTMODE` writer - PRS Start Mode"]
pub type PRSSTARTMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSSTARTMODE_A, 2, O>;
impl<'a, const O: u8> PRSSTARTMODE_W<'a, O> {
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSSTARTMODE_A::BOTH)
    }
}
#[doc = "Field `PRSSTOPMODE` reader - PRS Stop Mode"]
pub type PRSSTOPMODE_R = crate::FieldReader<u8, PRSSTOPMODE_A>;
#[doc = "PRS Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPMODE_A {
    #[doc = "0: PRS cannot stop the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can stop the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can stop the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    BOTH = 3,
}
impl From<PRSSTOPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPMODE_A) -> Self {
        variant as _
    }
}
impl PRSSTOPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSTOPMODE_A {
        match self.bits {
            0 => PRSSTOPMODE_A::NONE,
            1 => PRSSTOPMODE_A::RISING,
            2 => PRSSTOPMODE_A::FALLING,
            3 => PRSSTOPMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTOPMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTOPMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTOPMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTOPMODE_A::BOTH
    }
}
#[doc = "Field `PRSSTOPMODE` writer - PRS Stop Mode"]
pub type PRSSTOPMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSSTOPMODE_A, 2, O>;
impl<'a, const O: u8> PRSSTOPMODE_W<'a, O> {
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSSTOPMODE_A::BOTH)
    }
}
#[doc = "Field `PRSCLEARMODE` reader - PRS Clear Mode"]
pub type PRSCLEARMODE_R = crate::FieldReader<u8, PRSCLEARMODE_A>;
#[doc = "PRS Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARMODE_A {
    #[doc = "0: PRS cannot clear the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can clear the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can clear the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    BOTH = 3,
}
impl From<PRSCLEARMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARMODE_A) -> Self {
        variant as _
    }
}
impl PRSCLEARMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSCLEARMODE_A {
        match self.bits {
            0 => PRSCLEARMODE_A::NONE,
            1 => PRSCLEARMODE_A::RISING,
            2 => PRSCLEARMODE_A::FALLING,
            3 => PRSCLEARMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSCLEARMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSCLEARMODE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSCLEARMODE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSCLEARMODE_A::BOTH
    }
}
#[doc = "Field `PRSCLEARMODE` writer - PRS Clear Mode"]
pub type PRSCLEARMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSCLEARMODE_A, 2, O>;
impl<'a, const O: u8> PRSCLEARMODE_W<'a, O> {
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(PRSCLEARMODE_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:2 - PRS Start Select"]
    #[inline(always)]
    pub fn prsstartsel(&self) -> PRSSTARTSEL_R {
        PRSSTARTSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - PRS Stop Select"]
    #[inline(always)]
    pub fn prsstopsel(&self) -> PRSSTOPSEL_R {
        PRSSTOPSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PRS Clear Select"]
    #[inline(always)]
    pub fn prsclearsel(&self) -> PRSCLEARSEL_R {
        PRSCLEARSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    pub fn prsstartmode(&self) -> PRSSTARTMODE_R {
        PRSSTARTMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    pub fn prsstopmode(&self) -> PRSSTOPMODE_R {
        PRSSTOPMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    pub fn prsclearmode(&self) -> PRSCLEARMODE_R {
        PRSCLEARMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PRS Start Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartsel(&mut self) -> PRSSTARTSEL_W<0> {
        PRSSTARTSEL_W::new(self)
    }
    #[doc = "Bits 6:8 - PRS Stop Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopsel(&mut self) -> PRSSTOPSEL_W<6> {
        PRSSTOPSEL_W::new(self)
    }
    #[doc = "Bits 12:14 - PRS Clear Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearsel(&mut self) -> PRSCLEARSEL_W<12> {
        PRSCLEARSEL_W::new(self)
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartmode(&mut self) -> PRSSTARTMODE_W<18> {
        PRSSTARTMODE_W::new(self)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopmode(&mut self) -> PRSSTOPMODE_W<22> {
        PRSSTOPMODE_W::new(self)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearmode(&mut self) -> PRSCLEARMODE_W<26> {
        PRSCLEARMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRS Input Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prssel](index.html) module"]
pub struct PRSSEL_SPEC;
impl crate::RegisterSpec for PRSSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prssel::R](R) reader structure"]
impl crate::Readable for PRSSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prssel::W](W) writer structure"]
impl crate::Writable for PRSSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSSEL to value 0"]
impl crate::Resettable for PRSSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
