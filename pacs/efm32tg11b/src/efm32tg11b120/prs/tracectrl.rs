#[doc = "Register `TRACECTRL` reader"]
pub struct R(crate::R<TRACECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECTRL` writer"]
pub struct W(crate::W<TRACECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECTRL_SPEC>;
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
impl From<crate::W<TRACECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTARTEN` reader - PRS TSTART Enable"]
pub type TSTARTEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTARTEN` writer - PRS TSTART Enable"]
pub type TSTARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACECTRL_SPEC, bool, O>;
#[doc = "Field `TSTART` reader - MTB TSTART PRS Select"]
pub type TSTART_R = crate::FieldReader<u8, TSTART_A>;
#[doc = "MTB TSTART PRS Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART_A {
    #[doc = "0: PRS ch 0 is controlling TSTART."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 is controlling TSTART."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 is controlling TSTART."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 is controlling TSTART."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 is controlling TSTART."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 is controlling TSTART."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 is controlling TSTART."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 is controlling TSTART."]
    PRSCH7 = 7,
}
impl From<TSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as _
    }
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTART_A {
        match self.bits {
            0 => TSTART_A::PRSCH0,
            1 => TSTART_A::PRSCH1,
            2 => TSTART_A::PRSCH2,
            3 => TSTART_A::PRSCH3,
            4 => TSTART_A::PRSCH4,
            5 => TSTART_A::PRSCH5,
            6 => TSTART_A::PRSCH6,
            7 => TSTART_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTART_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTART_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTART_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTART_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTART_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTART_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TSTART_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TSTART_A::PRSCH7
    }
}
#[doc = "Field `TSTART` writer - MTB TSTART PRS Select"]
pub type TSTART_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRACECTRL_SPEC, u8, TSTART_A, 3, O>;
impl<'a, const O: u8> TSTART_W<'a, O> {
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH5)
    }
    #[doc = "PRS ch 6 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH6)
    }
    #[doc = "PRS ch 7 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(TSTART_A::PRSCH7)
    }
}
#[doc = "Field `TSTOPEN` reader - PRS TSTOP Enable"]
pub type TSTOPEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTOPEN` writer - PRS TSTOP Enable"]
pub type TSTOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRACECTRL_SPEC, bool, O>;
#[doc = "Field `TSTOP` reader - MTB TSTOP PRS Select"]
pub type TSTOP_R = crate::FieldReader<u8, TSTOP_A>;
#[doc = "MTB TSTOP PRS Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP_A {
    #[doc = "0: PRS ch 0 is controlling TSTOP."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 is controlling TSTOP."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 is controlling TSTOP."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 is controlling TSTOP."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 is controlling TSTOP."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 is controlling TSTOP."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 is controlling TSTOP."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 is controlling TSTOP."]
    PRSCH7 = 7,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        variant as _
    }
}
impl TSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTOP_A {
        match self.bits {
            0 => TSTOP_A::PRSCH0,
            1 => TSTOP_A::PRSCH1,
            2 => TSTOP_A::PRSCH2,
            3 => TSTOP_A::PRSCH3,
            4 => TSTOP_A::PRSCH4,
            5 => TSTOP_A::PRSCH5,
            6 => TSTOP_A::PRSCH6,
            7 => TSTOP_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTOP_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTOP_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTOP_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTOP_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTOP_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTOP_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TSTOP_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TSTOP_A::PRSCH7
    }
}
#[doc = "Field `TSTOP` writer - MTB TSTOP PRS Select"]
pub type TSTOP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRACECTRL_SPEC, u8, TSTOP_A, 3, O>;
impl<'a, const O: u8> TSTOP_W<'a, O> {
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH5)
    }
    #[doc = "PRS ch 6 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH6)
    }
    #[doc = "PRS ch 7 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(TSTOP_A::PRSCH7)
    }
}
impl R {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS Select"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS Select"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstarten(&mut self) -> TSTARTEN_W<0> {
        TSTARTEN_W::new(self)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS Select"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<1> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstopen(&mut self) -> TSTOPEN_W<8> {
        TSTOPEN_W::new(self)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS Select"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<9> {
        TSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Trace Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tracectrl](index.html) module"]
pub struct TRACECTRL_SPEC;
impl crate::RegisterSpec for TRACECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tracectrl::R](R) reader structure"]
impl crate::Readable for TRACECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tracectrl::W](W) writer structure"]
impl crate::Writable for TRACECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACECTRL to value 0"]
impl crate::Resettable for TRACECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
