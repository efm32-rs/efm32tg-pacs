#[doc = "Register `TIMECMP0` reader"]
pub struct R(crate::R<TIMECMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMECMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMECMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMECMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMECMP0` writer"]
pub struct W(crate::W<TIMECMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMECMP0_SPEC>;
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
impl From<crate::W<TIMECMP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMECMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCMPVAL` reader - Timer Comparator 0"]
pub type TCMPVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCMPVAL` writer - Timer Comparator 0"]
pub type TCMPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMECMP0_SPEC, u8, u8, 8, O>;
#[doc = "Field `TSTART` reader - Timer Start Source"]
pub type TSTART_R = crate::FieldReader<u8, TSTART_A>;
#[doc = "Timer Start Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART_A {
    #[doc = "0: Comparator 0 is disabled"]
    DISABLE = 0,
    #[doc = "1: Comparator 0 and timer are started at TX end of frame"]
    TXEOF = 1,
    #[doc = "2: Comparator 0 and timer are started at TX Complete"]
    TXC = 2,
    #[doc = "3: Comparator 0 and timer are started at RX going Active (default: low)"]
    RXACT = 3,
    #[doc = "4: Comparator 0 and timer are started at RX end of frame"]
    RXEOF = 4,
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
    pub fn variant(&self) -> Option<TSTART_A> {
        match self.bits {
            0 => Some(TSTART_A::DISABLE),
            1 => Some(TSTART_A::TXEOF),
            2 => Some(TSTART_A::TXC),
            3 => Some(TSTART_A::RXACT),
            4 => Some(TSTART_A::RXEOF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `TXEOF`"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TSTART_A::TXEOF
    }
    #[doc = "Checks if the value of the field is `TXC`"]
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == TSTART_A::TXC
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTART_A::RXACT
    }
    #[doc = "Checks if the value of the field is `RXEOF`"]
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTART_A::RXEOF
    }
}
#[doc = "Field `TSTART` writer - Timer Start Source"]
pub type TSTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMECMP0_SPEC, u8, TSTART_A, 3, O>;
impl<'a, const O: u8> TSTART_W<'a, O> {
    #[doc = "Comparator 0 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTART_A::DISABLE)
    }
    #[doc = "Comparator 0 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut W {
        self.variant(TSTART_A::TXEOF)
    }
    #[doc = "Comparator 0 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn txc(self) -> &'a mut W {
        self.variant(TSTART_A::TXC)
    }
    #[doc = "Comparator 0 and timer are started at RX going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTART_A::RXACT)
    }
    #[doc = "Comparator 0 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut W {
        self.variant(TSTART_A::RXEOF)
    }
}
#[doc = "Field `TSTOP` reader - Source Used to Disable Comparator 0"]
pub type TSTOP_R = crate::FieldReader<u8, TSTOP_A>;
#[doc = "Source Used to Disable Comparator 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP_A {
    #[doc = "0: Comparator 0 is disabled when the counter equals TCMPVAL and triggers a TCMP0 event"]
    TCMP0 = 0,
    #[doc = "1: Comparator 0 is disabled at the start of transmission"]
    TXST = 1,
    #[doc = "2: Comparator 0 is disabled on RX going going Active (default: low)"]
    RXACT = 2,
    #[doc = "3: Comparator 0 is disabled on RX going Inactive"]
    RXACTN = 3,
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
    pub fn variant(&self) -> Option<TSTOP_A> {
        match self.bits {
            0 => Some(TSTOP_A::TCMP0),
            1 => Some(TSTOP_A::TXST),
            2 => Some(TSTOP_A::RXACT),
            3 => Some(TSTOP_A::RXACTN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TSTOP_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TXST`"]
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == TSTOP_A::TXST
    }
    #[doc = "Checks if the value of the field is `RXACT`"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOP_A::RXACT
    }
    #[doc = "Checks if the value of the field is `RXACTN`"]
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOP_A::RXACTN
    }
}
#[doc = "Field `TSTOP` writer - Source Used to Disable Comparator 0"]
pub type TSTOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMECMP0_SPEC, u8, TSTOP_A, 3, O>;
impl<'a, const O: u8> TSTOP_W<'a, O> {
    #[doc = "Comparator 0 is disabled when the counter equals TCMPVAL and triggers a TCMP0 event"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(TSTOP_A::TCMP0)
    }
    #[doc = "Comparator 0 is disabled at the start of transmission"]
    #[inline(always)]
    pub fn txst(self) -> &'a mut W {
        self.variant(TSTOP_A::TXST)
    }
    #[doc = "Comparator 0 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut W {
        self.variant(TSTOP_A::RXACT)
    }
    #[doc = "Comparator 0 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut W {
        self.variant(TSTOP_A::RXACTN)
    }
}
#[doc = "Field `RESTARTEN` reader - Restart Timer on TCMP0"]
pub type RESTARTEN_R = crate::BitReader<bool>;
#[doc = "Field `RESTARTEN` writer - Restart Timer on TCMP0"]
pub type RESTARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMECMP0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Timer Comparator 0"]
    #[inline(always)]
    pub fn tcmpval(&self) -> TCMPVAL_R {
        TCMPVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 0"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP0"]
    #[inline(always)]
    pub fn restarten(&self) -> RESTARTEN_R {
        RESTARTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpval(&mut self) -> TCMPVAL_W<0> {
        TCMPVAL_W::new(self)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<16> {
        TSTART_W::new(self)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<20> {
        TSTOP_W::new(self)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP0"]
    #[inline(always)]
    #[must_use]
    pub fn restarten(&mut self) -> RESTARTEN_W<24> {
        RESTARTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used to Generate Interrupts and Various Delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecmp0](index.html) module"]
pub struct TIMECMP0_SPEC;
impl crate::RegisterSpec for TIMECMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timecmp0::R](R) reader structure"]
impl crate::Readable for TIMECMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timecmp0::W](W) writer structure"]
impl crate::Writable for TIMECMP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMECMP0 to value 0"]
impl crate::Resettable for TIMECMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
