#[doc = "Register `CALCTRL` reader"]
pub struct R(crate::R<CALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCTRL` writer"]
pub struct W(crate::W<CALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCTRL_SPEC>;
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
impl From<crate::W<CALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub type UPSEL_R = crate::FieldReader<u8, UPSEL_A>;
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPSEL_A {
    #[doc = "0: Select HFXO as up-counter"]
    HFXO = 0,
    #[doc = "1: Select LFXO as up-counter"]
    LFXO = 1,
    #[doc = "2: Select HFRCO as up-counter"]
    HFRCO = 2,
    #[doc = "3: Select LFRCO as up-counter"]
    LFRCO = 3,
    #[doc = "4: Select AUXHFRCO as up-counter"]
    AUXHFRCO = 4,
    #[doc = "5: Select PRS input selected by PRSUPSEL as up-counter"]
    PRS = 5,
}
impl From<UPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL_A) -> Self {
        variant as _
    }
}
impl UPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPSEL_A> {
        match self.bits {
            0 => Some(UPSEL_A::HFXO),
            1 => Some(UPSEL_A::LFXO),
            2 => Some(UPSEL_A::HFRCO),
            3 => Some(UPSEL_A::LFRCO),
            4 => Some(UPSEL_A::AUXHFRCO),
            5 => Some(UPSEL_A::PRS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == UPSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == UPSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == UPSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == UPSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == UPSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == UPSEL_A::PRS
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub type UPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALCTRL_SPEC, u8, UPSEL_A, 3, O>;
impl<'a, const O: u8> UPSEL_W<'a, O> {
    #[doc = "Select HFXO as up-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::HFXO)
    }
    #[doc = "Select LFXO as up-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::LFXO)
    }
    #[doc = "Select HFRCO as up-counter"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::AUXHFRCO)
    }
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(UPSEL_A::PRS)
    }
}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub type DOWNSEL_R = crate::FieldReader<u8, DOWNSEL_A>;
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOWNSEL_A {
    #[doc = "0: Select HFCLK for down-counter"]
    HFCLK = 0,
    #[doc = "1: Select HFXO for down-counter"]
    HFXO = 1,
    #[doc = "2: Select LFXO for down-counter"]
    LFXO = 2,
    #[doc = "3: Select HFRCO for down-counter"]
    HFRCO = 3,
    #[doc = "4: Select LFRCO for down-counter"]
    LFRCO = 4,
    #[doc = "5: Select AUXHFRCO for down-counter"]
    AUXHFRCO = 5,
    #[doc = "6: Select PRS input selected by PRSDOWNSEL as down-counter"]
    PRS = 6,
}
impl From<DOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL_A) -> Self {
        variant as _
    }
}
impl DOWNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOWNSEL_A> {
        match self.bits {
            0 => Some(DOWNSEL_A::HFCLK),
            1 => Some(DOWNSEL_A::HFXO),
            2 => Some(DOWNSEL_A::LFXO),
            3 => Some(DOWNSEL_A::HFRCO),
            4 => Some(DOWNSEL_A::LFRCO),
            5 => Some(DOWNSEL_A::AUXHFRCO),
            6 => Some(DOWNSEL_A::PRS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DOWNSEL_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == DOWNSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == DOWNSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == DOWNSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == DOWNSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DOWNSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == DOWNSEL_A::PRS
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub type DOWNSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CALCTRL_SPEC, u8, DOWNSEL_A, 3, O>;
impl<'a, const O: u8> DOWNSEL_W<'a, O> {
    #[doc = "Select HFCLK for down-counter"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFCLK)
    }
    #[doc = "Select HFXO for down-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFXO)
    }
    #[doc = "Select LFXO for down-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFXO)
    }
    #[doc = "Select HFRCO for down-counter"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::AUXHFRCO)
    }
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(DOWNSEL_A::PRS)
    }
}
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALCTRL_SPEC, bool, O>;
#[doc = "Field `PRSUPSEL` reader - PRS Select for PRS Input When Selected in UPSEL"]
pub type PRSUPSEL_R = crate::FieldReader<u8, PRSUPSEL_A>;
#[doc = "PRS Select for PRS Input When Selected in UPSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSUPSEL_A {
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
impl From<PRSUPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSUPSEL_A) -> Self {
        variant as _
    }
}
impl PRSUPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSUPSEL_A {
        match self.bits {
            0 => PRSUPSEL_A::PRSCH0,
            1 => PRSUPSEL_A::PRSCH1,
            2 => PRSUPSEL_A::PRSCH2,
            3 => PRSUPSEL_A::PRSCH3,
            4 => PRSUPSEL_A::PRSCH4,
            5 => PRSUPSEL_A::PRSCH5,
            6 => PRSUPSEL_A::PRSCH6,
            7 => PRSUPSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSUPSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSUPSEL` writer - PRS Select for PRS Input When Selected in UPSEL"]
pub type PRSUPSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CALCTRL_SPEC, u8, PRSUPSEL_A, 3, O>;
impl<'a, const O: u8> PRSUPSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSUPSEL_A::PRSCH7)
    }
}
#[doc = "Field `PRSDOWNSEL` reader - PRS Select for PRS Input When Selected in DOWNSEL"]
pub type PRSDOWNSEL_R = crate::FieldReader<u8, PRSDOWNSEL_A>;
#[doc = "PRS Select for PRS Input When Selected in DOWNSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSDOWNSEL_A {
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
impl From<PRSDOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSDOWNSEL_A) -> Self {
        variant as _
    }
}
impl PRSDOWNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSDOWNSEL_A {
        match self.bits {
            0 => PRSDOWNSEL_A::PRSCH0,
            1 => PRSDOWNSEL_A::PRSCH1,
            2 => PRSDOWNSEL_A::PRSCH2,
            3 => PRSDOWNSEL_A::PRSCH3,
            4 => PRSDOWNSEL_A::PRSCH4,
            5 => PRSDOWNSEL_A::PRSCH5,
            6 => PRSDOWNSEL_A::PRSCH6,
            7 => PRSDOWNSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSDOWNSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSDOWNSEL` writer - PRS Select for PRS Input When Selected in DOWNSEL"]
pub type PRSDOWNSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CALCTRL_SPEC, u8, PRSDOWNSEL_A, 3, O>;
impl<'a, const O: u8> PRSDOWNSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSDOWNSEL_A::PRSCH7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DOWNSEL_R {
        DOWNSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline(always)]
    pub fn prsupsel(&self) -> PRSUPSEL_R {
        PRSUPSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline(always)]
    pub fn prsdownsel(&self) -> PRSDOWNSEL_R {
        PRSDOWNSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    #[must_use]
    pub fn upsel(&mut self) -> UPSEL_W<0> {
        UPSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Calibration Down-counter Select"]
    #[inline(always)]
    #[must_use]
    pub fn downsel(&mut self) -> DOWNSEL_W<4> {
        DOWNSEL_W::new(self)
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<8> {
        CONT_W::new(self)
    }
    #[doc = "Bits 16:18 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn prsupsel(&mut self) -> PRSUPSEL_W<16> {
        PRSUPSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline(always)]
    #[must_use]
    pub fn prsdownsel(&mut self) -> PRSDOWNSEL_W<24> {
        PRSDOWNSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calctrl](index.html) module"]
pub struct CALCTRL_SPEC;
impl crate::RegisterSpec for CALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calctrl::R](R) reader structure"]
impl crate::Readable for CALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calctrl::W](W) writer structure"]
impl crate::Writable for CALCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
