#[doc = "Register `SINGLECTRL` reader"]
pub struct R(crate::R<SINGLECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINGLECTRL` writer"]
pub struct W(crate::W<SINGLECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLECTRL_SPEC>;
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
impl From<crate::W<SINGLECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - Single Channel Repetitive Mode"]
pub type REP_R = crate::BitReader<bool>;
#[doc = "Field `REP` writer - Single Channel Repetitive Mode"]
pub type REP_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 0>;
#[doc = "Field `DIFF` reader - Single Channel Differential Mode"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Single Channel Differential Mode"]
pub type DIFF_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 1>;
#[doc = "Field `ADJ` reader - Single Channel Result Adjustment"]
pub type ADJ_R = crate::BitReader<bool>;
#[doc = "Field `ADJ` writer - Single Channel Result Adjustment"]
pub type ADJ_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 2>;
#[doc = "Single Channel Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution."]
    _12BIT = 0,
    #[doc = "1: 8-bit resolution."]
    _8BIT = 1,
    #[doc = "2: 6-bit resolution."]
    _6BIT = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    OVS = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES` reader - Single Channel Resolution Select"]
pub type RES_R = crate::FieldReader<u8, RES_A>;
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
#[doc = "Field `RES` writer - Single Channel Resolution Select"]
pub type RES_W<'a> = crate::FieldWriterSafe<'a, u32, SINGLECTRL_SPEC, u8, RES_A, 2, 3>;
impl<'a> RES_W<'a> {
    #[doc = "12-bit resolution."]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RES_A::_12BIT)
    }
    #[doc = "8-bit resolution."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "6-bit resolution."]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RES_A::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RES_A::OVS)
    }
}
#[doc = "Single Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[doc = "6: VFS = 2xAVDD with AVDD as the reference source"]
    _2XVDD = 6,
    #[doc = "7: Use SINGLECTRLX to configure reference"]
    CONF = 7,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REF` reader - Single Channel Reference Selection"]
pub type REF_R = crate::FieldReader<u8, REF_A>;
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
#[doc = "Field `REF` writer - Single Channel Reference Selection"]
pub type REF_W<'a> = crate::FieldWriterSafe<'a, u32, SINGLECTRL_SPEC, u8, REF_A, 3, 5>;
impl<'a> REF_W<'a> {
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
    #[doc = "VFS = 2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REF_A::_2XVDD)
    }
    #[doc = "Use SINGLECTRLX to configure reference"]
    #[inline(always)]
    pub fn conf(self) -> &'a mut W {
        self.variant(REF_A::CONF)
    }
}
#[doc = "Field `POSSEL` reader - Single Channel Positive Input Selection"]
pub type POSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSSEL` writer - Single Channel Positive Input Selection"]
pub type POSSEL_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, u8, 8, 8>;
#[doc = "Field `NEGSEL` reader - Single Channel Negative Input Selection"]
pub type NEGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEGSEL` writer - Single Channel Negative Input Selection"]
pub type NEGSEL_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, u8, 8, 16>;
#[doc = "Single Channel Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AT_A {
    #[doc = "0: 1 conversion clock cycle acquisition time for single channel"]
    _1CYCLE = 0,
    #[doc = "1: 2 conversion clock cycles acquisition time for single channel"]
    _2CYCLES = 1,
    #[doc = "2: 3 conversion clock cycles acquisition time for single channel"]
    _3CYCLES = 2,
    #[doc = "3: 4 conversion clock cycles acquisition time for single channel"]
    _4CYCLES = 3,
    #[doc = "4: 8 conversion clock cycles acquisition time for single channel"]
    _8CYCLES = 4,
    #[doc = "5: 16 conversion clock cycles acquisition time for single channel"]
    _16CYCLES = 5,
    #[doc = "6: 32 conversion clock cycles acquisition time for single channel"]
    _32CYCLES = 6,
    #[doc = "7: 64 conversion clock cycles acquisition time for single channel"]
    _64CYCLES = 7,
    #[doc = "8: 128 conversion clock cycles acquisition time for single channel"]
    _128CYCLES = 8,
    #[doc = "9: 256 conversion clock cycles acquisition time for single channel"]
    _256CYCLES = 9,
}
impl From<AT_A> for u8 {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AT` reader - Single Channel Acquisition Time"]
pub type AT_R = crate::FieldReader<u8, AT_A>;
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
#[doc = "Field `AT` writer - Single Channel Acquisition Time"]
pub type AT_W<'a> = crate::FieldWriter<'a, u32, SINGLECTRL_SPEC, u8, AT_A, 4, 24>;
impl<'a> AT_W<'a> {
    #[doc = "1 conversion clock cycle acquisition time for single channel"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(AT_A::_1CYCLE)
    }
    #[doc = "2 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(AT_A::_2CYCLES)
    }
    #[doc = "3 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut W {
        self.variant(AT_A::_3CYCLES)
    }
    #[doc = "4 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(AT_A::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(AT_A::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(AT_A::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(AT_A::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(AT_A::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(AT_A::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(AT_A::_256CYCLES)
    }
}
#[doc = "Field `PRSEN` reader - Single Channel PRS Trigger Enable"]
pub type PRSEN_R = crate::BitReader<bool>;
#[doc = "Field `PRSEN` writer - Single Channel PRS Trigger Enable"]
pub type PRSEN_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 29>;
#[doc = "Field `CMPEN` reader - Compare Logic Enable for Single Channel"]
pub type CMPEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPEN` writer - Compare Logic Enable for Single Channel"]
pub type CMPEN_W<'a> = crate::BitWriter<'a, u32, SINGLECTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Single Channel Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Channel Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Channel Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Single Channel Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Single Channel Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Single Channel Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Single Channel Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Single Channel PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Single Channel"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Channel Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W::new(self)
    }
    #[doc = "Bit 1 - Single Channel Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W::new(self)
    }
    #[doc = "Bit 2 - Single Channel Result Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W::new(self)
    }
    #[doc = "Bits 3:4 - Single Channel Resolution Select"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W::new(self)
    }
    #[doc = "Bits 5:7 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W::new(self)
    }
    #[doc = "Bits 8:15 - Single Channel Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Single Channel Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W::new(self)
    }
    #[doc = "Bits 24:27 - Single Channel Acquisition Time"]
    #[inline(always)]
    pub fn at(&mut self) -> AT_W {
        AT_W::new(self)
    }
    #[doc = "Bit 29 - Single Channel PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PRSEN_W {
        PRSEN_W::new(self)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Single Channel"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlectrl](index.html) module"]
pub struct SINGLECTRL_SPEC;
impl crate::RegisterSpec for SINGLECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlectrl::R](R) reader structure"]
impl crate::Readable for SINGLECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singlectrl::W](W) writer structure"]
impl crate::Writable for SINGLECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SINGLECTRL to value 0x00ff_ff00"]
impl crate::Resettable for SINGLECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ff00
    }
}
