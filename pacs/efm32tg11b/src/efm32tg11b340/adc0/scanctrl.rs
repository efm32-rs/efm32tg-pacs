#[doc = "Register `SCANCTRL` reader"]
pub struct R(crate::R<SCANCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANCTRL` writer"]
pub struct W(crate::W<SCANCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANCTRL_SPEC>;
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
impl From<crate::W<SCANCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - Scan Sequence Repetitive Mode"]
pub type REP_R = crate::BitReader<bool>;
#[doc = "Field `REP` writer - Scan Sequence Repetitive Mode"]
pub type REP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRL_SPEC, bool, O>;
#[doc = "Field `DIFF` reader - Scan Sequence Differential Mode"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Scan Sequence Differential Mode"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRL_SPEC, bool, O>;
#[doc = "Field `ADJ` reader - Scan Sequence Result Adjustment"]
pub type ADJ_R = crate::BitReader<bool>;
#[doc = "Field `ADJ` writer - Scan Sequence Result Adjustment"]
pub type ADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRL_SPEC, bool, O>;
#[doc = "Field `RES` reader - Scan Sequence Resolution Select"]
pub type RES_R = crate::FieldReader<u8, RES_A>;
#[doc = "Scan Sequence Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution"]
    _12BIT = 0,
    #[doc = "1: 8-bit resolution"]
    _8BIT = 1,
    #[doc = "2: 6-bit resolution"]
    _6BIT = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::_12BIT,
            1 => RES_A::_8BIT,
            2 => RES_A::_6BIT,
            3 => RES_A::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES_A::_6BIT
    }
    #[doc = "Checks if the value of the field is `OVS`"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES_A::OVS
    }
}
#[doc = "Field `RES` writer - Scan Sequence Resolution Select"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SCANCTRL_SPEC, u8, RES_A, 2, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RES_A::_12BIT)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RES_A::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RES_A::OVS)
    }
}
#[doc = "Field `REF` reader - Scan Sequence Reference Selection"]
pub type REF_R = crate::FieldReader<u8, REF_A>;
#[doc = "Scan Sequence Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_A {
    #[doc = "0: VFS = 1.25V with internal VBGR reference"]
    _1V25 = 0,
    #[doc = "1: VFS = 2.5V with internal VBGR reference"]
    _2V5 = 1,
    #[doc = "2: VFS = AVDD with AVDD as reference source"]
    VDD = 2,
    #[doc = "3: VFS = 5V with internal VBGR reference"]
    _5V = 3,
    #[doc = "4: Single ended external reference"]
    EXTSINGLE = 4,
    #[doc = "5: Differential external reference, 2x"]
    _2XEXTDIFF = 5,
    #[doc = "6: VFS=2xAVDD with AVDD as the reference source"]
    _2XVDD = 6,
    #[doc = "7: Use SCANCTRLX to configure reference"]
    CONF = 7,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_A {
        match self.bits {
            0 => REF_A::_1V25,
            1 => REF_A::_2V5,
            2 => REF_A::VDD,
            3 => REF_A::_5V,
            4 => REF_A::EXTSINGLE,
            5 => REF_A::_2XEXTDIFF,
            6 => REF_A::_2XVDD,
            7 => REF_A::CONF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF_A::VDD
    }
    #[doc = "Checks if the value of the field is `_5V`"]
    #[inline(always)]
    pub fn is_5v(&self) -> bool {
        *self == REF_A::_5V
    }
    #[doc = "Checks if the value of the field is `EXTSINGLE`"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF_A::EXTSINGLE
    }
    #[doc = "Checks if the value of the field is `_2XEXTDIFF`"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF_A::_2XEXTDIFF
    }
    #[doc = "Checks if the value of the field is `_2XVDD`"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF_A::_2XVDD
    }
    #[doc = "Checks if the value of the field is `CONF`"]
    #[inline(always)]
    pub fn is_conf(&self) -> bool {
        *self == REF_A::CONF
    }
}
#[doc = "Field `REF` writer - Scan Sequence Reference Selection"]
pub type REF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SCANCTRL_SPEC, u8, REF_A, 3, O>;
impl<'a, const O: u8> REF_W<'a, O> {
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REF_A::_1V25)
    }
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REF_A::_2V5)
    }
    #[doc = "VFS = AVDD with AVDD as reference source"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REF_A::VDD)
    }
    #[doc = "VFS = 5V with internal VBGR reference"]
    #[inline(always)]
    pub fn _5v(self) -> &'a mut W {
        self.variant(REF_A::_5V)
    }
    #[doc = "Single ended external reference"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut W {
        self.variant(REF_A::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut W {
        self.variant(REF_A::_2XEXTDIFF)
    }
    #[doc = "VFS=2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REF_A::_2XVDD)
    }
    #[doc = "Use SCANCTRLX to configure reference"]
    #[inline(always)]
    pub fn conf(self) -> &'a mut W {
        self.variant(REF_A::CONF)
    }
}
#[doc = "Field `AT` reader - Scan Acquisition Time"]
pub type AT_R = crate::FieldReader<u8, AT_A>;
#[doc = "Scan Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AT_A {
    #[doc = "0: 1 conversion clock cycle acquisition time for scan"]
    _1CYCLE = 0,
    #[doc = "1: 2 conversion clock cycles acquisition time for scan"]
    _2CYCLES = 1,
    #[doc = "2: 3 conversion clock cycles acquisition time for scan"]
    _3CYCLES = 2,
    #[doc = "3: 4 conversion clock cycles acquisition time for scan"]
    _4CYCLES = 3,
    #[doc = "4: 8 conversion clock cycles acquisition time for scan"]
    _8CYCLES = 4,
    #[doc = "5: 16 conversion clock cycles acquisition time for scan"]
    _16CYCLES = 5,
    #[doc = "6: 32 conversion clock cycles acquisition time for scan"]
    _32CYCLES = 6,
    #[doc = "7: 64 conversion clock cycles acquisition time for scan"]
    _64CYCLES = 7,
    #[doc = "8: 128 conversion clock cycles acquisition time for scan"]
    _128CYCLES = 8,
    #[doc = "9: 256 conversion clock cycles acquisition time for scan"]
    _256CYCLES = 9,
}
impl From<AT_A> for u8 {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        variant as _
    }
}
impl AT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AT_A> {
        match self.bits {
            0 => Some(AT_A::_1CYCLE),
            1 => Some(AT_A::_2CYCLES),
            2 => Some(AT_A::_3CYCLES),
            3 => Some(AT_A::_4CYCLES),
            4 => Some(AT_A::_8CYCLES),
            5 => Some(AT_A::_16CYCLES),
            6 => Some(AT_A::_32CYCLES),
            7 => Some(AT_A::_64CYCLES),
            8 => Some(AT_A::_128CYCLES),
            9 => Some(AT_A::_256CYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1CYCLE`"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT_A::_1CYCLE
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_3CYCLES`"]
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == AT_A::_3CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT_A::_256CYCLES
    }
}
#[doc = "Field `AT` writer - Scan Acquisition Time"]
pub type AT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCANCTRL_SPEC, u8, AT_A, 4, O>;
impl<'a, const O: u8> AT_W<'a, O> {
    #[doc = "1 conversion clock cycle acquisition time for scan"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(AT_A::_1CYCLE)
    }
    #[doc = "2 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(AT_A::_2CYCLES)
    }
    #[doc = "3 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut W {
        self.variant(AT_A::_3CYCLES)
    }
    #[doc = "4 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(AT_A::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(AT_A::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(AT_A::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(AT_A::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(AT_A::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(AT_A::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(AT_A::_256CYCLES)
    }
}
#[doc = "Field `PRSEN` reader - Scan Sequence PRS Trigger Enable"]
pub type PRSEN_R = crate::BitReader<bool>;
#[doc = "Field `PRSEN` writer - Scan Sequence PRS Trigger Enable"]
pub type PRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRL_SPEC, bool, O>;
#[doc = "Field `CMPEN` reader - Compare Logic Enable for Scan"]
pub type CMPEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPEN` writer - Compare Logic Enable for Scan"]
pub type CMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Scan Sequence Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Scan Sequence Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Scan Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Scan Sequence PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Scan"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<0> {
        REP_W::new(self)
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<1> {
        DIFF_W::new(self)
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<2> {
        ADJ_W::new(self)
    }
    #[doc = "Bits 3:4 - Scan Sequence Resolution Select"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Bits 5:7 - Scan Sequence Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<5> {
        REF_W::new(self)
    }
    #[doc = "Bits 24:27 - Scan Acquisition Time"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AT_W<24> {
        AT_W::new(self)
    }
    #[doc = "Bit 29 - Scan Sequence PRS Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsen(&mut self) -> PRSEN_W<29> {
        PRSEN_W::new(self)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Scan"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<31> {
        CMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanctrl](index.html) module"]
pub struct SCANCTRL_SPEC;
impl crate::RegisterSpec for SCANCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanctrl::R](R) reader structure"]
impl crate::Readable for SCANCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanctrl::W](W) writer structure"]
impl crate::Writable for SCANCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANCTRL to value 0"]
impl crate::Resettable for SCANCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
