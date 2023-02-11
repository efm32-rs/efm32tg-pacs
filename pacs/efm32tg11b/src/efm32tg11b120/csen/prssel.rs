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
#[doc = "Field `PRSSEL` reader - PRS Channel Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
#[doc = "PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as the start trigger"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as the start trigger"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as the start trigger"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as the start trigger"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as the start trigger"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as the start trigger"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as the start trigger"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as the start trigger"]
    PRSCH7 = 7,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL_A {
        match self.bits {
            0 => PRSSEL_A::PRSCH0,
            1 => PRSSEL_A::PRSCH1,
            2 => PRSSEL_A::PRSCH2,
            3 => PRSSEL_A::PRSCH3,
            4 => PRSSEL_A::PRSCH4,
            5 => PRSSEL_A::PRSCH5,
            6 => PRSSEL_A::PRSCH6,
            7 => PRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSSEL` writer - PRS Channel Select"]
pub type PRSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRSSEL_SPEC, u8, PRSSEL_A, 3, O>;
impl<'a, const O: u8> PRSSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
}
impl R {
    #[doc = "Bits 0:2 - PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<0> {
        PRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRS Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prssel](index.html) module"]
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
