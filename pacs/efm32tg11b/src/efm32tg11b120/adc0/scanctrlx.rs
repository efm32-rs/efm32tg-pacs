#[doc = "Register `SCANCTRLX` reader"]
pub struct R(crate::R<SCANCTRLX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANCTRLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANCTRLX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANCTRLX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANCTRLX` writer"]
pub struct W(crate::W<SCANCTRLX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANCTRLX_SPEC>;
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
impl From<crate::W<SCANCTRLX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANCTRLX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFSEL` reader - Scan Channel Reference Selection"]
pub type VREFSEL_R = crate::FieldReader<u8, VREFSEL_A>;
#[doc = "Scan Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFSEL_A {
    #[doc = "0: Internal 0.83V Bandgap reference"]
    VBGR = 0,
    #[doc = "1: Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    VDDXWATT = 1,
    #[doc = "2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    VREFPWATT = 2,
    #[doc = "3: Raw single ended external Vref: ADCn_EXTP"]
    VREFP = 3,
    #[doc = "5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    VREFPNWATT = 5,
    #[doc = "6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    VREFPN = 6,
    #[doc = "7: Internal Bandgap reference at low setting 0.78V"]
    VBGRLOW = 7,
}
impl From<VREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFSEL_A) -> Self {
        variant as _
    }
}
impl VREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREFSEL_A> {
        match self.bits {
            0 => Some(VREFSEL_A::VBGR),
            1 => Some(VREFSEL_A::VDDXWATT),
            2 => Some(VREFSEL_A::VREFPWATT),
            3 => Some(VREFSEL_A::VREFP),
            5 => Some(VREFSEL_A::VREFPNWATT),
            6 => Some(VREFSEL_A::VREFPN),
            7 => Some(VREFSEL_A::VBGRLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL_A::VBGR
    }
    #[doc = "Checks if the value of the field is `VDDXWATT`"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL_A::VDDXWATT
    }
    #[doc = "Checks if the value of the field is `VREFPWATT`"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPWATT
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL_A::VREFP
    }
    #[doc = "Checks if the value of the field is `VREFPNWATT`"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPNWATT
    }
    #[doc = "Checks if the value of the field is `VREFPN`"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL_A::VREFPN
    }
    #[doc = "Checks if the value of the field is `VBGRLOW`"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL_A::VBGRLOW
    }
}
#[doc = "Field `VREFSEL` writer - Scan Channel Reference Selection"]
pub type VREFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCANCTRLX_SPEC, u8, VREFSEL_A, 3, O>;
impl<'a, const O: u8> VREFSEL_W<'a, O> {
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(VREFSEL_A::VBGR)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VDDXWATT)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPWATT)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFP)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPNWATT)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut W {
        self.variant(VREFSEL_A::VREFPN)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut W {
        self.variant(VREFSEL_A::VBGRLOW)
    }
}
#[doc = "Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_R = crate::BitReader<bool>;
#[doc = "Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRLX_SPEC, bool, O>;
#[doc = "Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCANCTRLX_SPEC, u8, u8, 4, O>;
#[doc = "Field `VINATT` reader - Code for VIN Attenuation Factor"]
pub type VINATT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VINATT` writer - Code for VIN Attenuation Factor"]
pub type VINATT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCANCTRLX_SPEC, u8, u8, 4, O>;
#[doc = "Field `DVL` reader - Scan DV Level Select"]
pub type DVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVL` writer - Scan DV Level Select"]
pub type DVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCANCTRLX_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIFOOFACT` reader - Scan FIFO Overflow Action"]
pub type FIFOOFACT_R = crate::BitReader<bool>;
#[doc = "Field `FIFOOFACT` writer - Scan FIFO Overflow Action"]
pub type FIFOOFACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRLX_SPEC, bool, O>;
#[doc = "Field `PRSMODE` reader - Scan PRS Trigger Mode"]
pub type PRSMODE_R = crate::BitReader<bool>;
#[doc = "Field `PRSMODE` writer - Scan PRS Trigger Mode"]
pub type PRSMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRLX_SPEC, bool, O>;
#[doc = "Field `PRSSEL` reader - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
#[doc = "Scan Sequence PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers scan sequence"]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers scan sequence"]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers scan sequence"]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers scan sequence"]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers scan sequence"]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers scan sequence"]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers scan sequence"]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers scan sequence"]
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
#[doc = "Field `PRSSEL` writer - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCANCTRLX_SPEC, u8, PRSSEL_A, 3, O>;
impl<'a, const O: u8> PRSSEL_W<'a, O> {
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
}
#[doc = "Field `CONVSTARTDELAY` reader - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONVSTARTDELAY` writer - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCANCTRLX_SPEC, u8, u8, 5, O>;
#[doc = "Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_R = crate::BitReader<bool>;
#[doc = "Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCANCTRLX_SPEC, bool, O>;
#[doc = "Field `REPDELAY` reader - REPDELAY Select for SCAN REP Mode"]
pub type REPDELAY_R = crate::FieldReader<u8, REPDELAY_A>;
#[doc = "REPDELAY Select for SCAN REP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPDELAY_A {
    #[doc = "0: No delay"]
    NODELAY = 0,
    #[doc = "1: 4 conversion clock cycles"]
    _4CYCLES = 1,
    #[doc = "2: 8 conversion clock cycles"]
    _8CYCLES = 2,
    #[doc = "3: 16 conversion clock cycles"]
    _16CYCLES = 3,
    #[doc = "4: 32 conversion clock cycles"]
    _32CYCLES = 4,
    #[doc = "5: 64 conversion clock cycles"]
    _64CYCLES = 5,
    #[doc = "6: 128 conversion clock cycles"]
    _128CYCLES = 6,
    #[doc = "7: 256 conversion clock cycles"]
    _256CYCLES = 7,
}
impl From<REPDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: REPDELAY_A) -> Self {
        variant as _
    }
}
impl REPDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPDELAY_A {
        match self.bits {
            0 => REPDELAY_A::NODELAY,
            1 => REPDELAY_A::_4CYCLES,
            2 => REPDELAY_A::_8CYCLES,
            3 => REPDELAY_A::_16CYCLES,
            4 => REPDELAY_A::_32CYCLES,
            5 => REPDELAY_A::_64CYCLES,
            6 => REPDELAY_A::_128CYCLES,
            7 => REPDELAY_A::_256CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NODELAY`"]
    #[inline(always)]
    pub fn is_nodelay(&self) -> bool {
        *self == REPDELAY_A::NODELAY
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == REPDELAY_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == REPDELAY_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == REPDELAY_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == REPDELAY_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == REPDELAY_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == REPDELAY_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == REPDELAY_A::_256CYCLES
    }
}
#[doc = "Field `REPDELAY` writer - REPDELAY Select for SCAN REP Mode"]
pub type REPDELAY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCANCTRLX_SPEC, u8, REPDELAY_A, 3, O>;
impl<'a, const O: u8> REPDELAY_W<'a, O> {
    #[doc = "No delay"]
    #[inline(always)]
    pub fn nodelay(self) -> &'a mut W {
        self.variant(REPDELAY_A::NODELAY)
    }
    #[doc = "4 conversion clock cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(REPDELAY_A::_256CYCLES)
    }
}
impl R {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&self) -> VREFATTFIX_R {
        VREFATTFIX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&self) -> VREFATT_R {
        VREFATT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&self) -> VINATT_R {
        VINATT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&self) -> FIFOOFACT_R {
        FIFOOFACT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 22:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&self) -> CONVSTARTDELAY_R {
        CONVSTARTDELAY_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&self) -> CONVSTARTDELAYEN_R {
        CONVSTARTDELAYEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SCAN REP Mode"]
    #[inline(always)]
    pub fn repdelay(&self) -> REPDELAY_R {
        REPDELAY_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scan Channel Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<0> {
        VREFSEL_W::new(self)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefattfix(&mut self) -> VREFATTFIX_W<3> {
        VREFATTFIX_W::new(self)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    #[must_use]
    pub fn vrefatt(&mut self) -> VREFATT_W<4> {
        VREFATT_W::new(self)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn vinatt(&mut self) -> VINATT_W<8> {
        VINATT_W::new(self)
    }
    #[doc = "Bits 12:13 - Scan DV Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DVL_W<12> {
        DVL_W::new(self)
    }
    #[doc = "Bit 14 - Scan FIFO Overflow Action"]
    #[inline(always)]
    #[must_use]
    pub fn fifoofact(&mut self) -> FIFOOFACT_W<14> {
        FIFOOFACT_W::new(self)
    }
    #[doc = "Bit 16 - Scan PRS Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PRSMODE_W<16> {
        PRSMODE_W::new(self)
    }
    #[doc = "Bits 17:19 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<17> {
        PRSSEL_W::new(self)
    }
    #[doc = "Bits 22:26 - Delay Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelay(&mut self) -> CONVSTARTDELAY_W<22> {
        CONVSTARTDELAY_W::new(self)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelayen(&mut self) -> CONVSTARTDELAYEN_W<27> {
        CONVSTARTDELAYEN_W::new(self)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SCAN REP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn repdelay(&mut self) -> REPDELAY_W<29> {
        REPDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Control Register Continued\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanctrlx](index.html) module"]
pub struct SCANCTRLX_SPEC;
impl crate::RegisterSpec for SCANCTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanctrlx::R](R) reader structure"]
impl crate::Readable for SCANCTRLX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanctrlx::W](W) writer structure"]
impl crate::Writable for SCANCTRLX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANCTRLX to value 0"]
impl crate::Resettable for SCANCTRLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
