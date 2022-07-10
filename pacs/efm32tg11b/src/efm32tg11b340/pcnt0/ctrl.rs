#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: The module is disabled."]
    DISABLE = 0,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM3)."]
    OVSSINGLE = 1,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE = 2,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD = 3,
    #[doc = "4: LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    OVSQUAD1X = 4,
    #[doc = "5: LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    OVSQUAD2X = 5,
    #[doc = "6: LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    OVSQUAD4X = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Mode Select"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLE),
            1 => Some(MODE_A::OVSSINGLE),
            2 => Some(MODE_A::EXTCLKSINGLE),
            3 => Some(MODE_A::EXTCLKQUAD),
            4 => Some(MODE_A::OVSQUAD1X),
            5 => Some(MODE_A::OVSQUAD2X),
            6 => Some(MODE_A::OVSQUAD4X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `OVSSINGLE`"]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE_A::OVSSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKSINGLE`"]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE_A::EXTCLKSINGLE
    }
    #[doc = "Checks if the value of the field is `EXTCLKQUAD`"]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE_A::EXTCLKQUAD
    }
    #[doc = "Checks if the value of the field is `OVSQUAD1X`"]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == MODE_A::OVSQUAD1X
    }
    #[doc = "Checks if the value of the field is `OVSQUAD2X`"]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == MODE_A::OVSQUAD2X
    }
    #[doc = "Checks if the value of the field is `OVSQUAD4X`"]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == MODE_A::OVSQUAD4X
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 3, 0>;
impl<'a> MODE_W<'a> {
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut W {
        self.variant(MODE_A::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut W {
        self.variant(MODE_A::EXTCLKQUAD)
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD1X)
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD2X)
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut W {
        self.variant(MODE_A::OVSQUAD4X)
    }
}
#[doc = "Field `FILT` reader - Enable Digital Pulse Width Filter"]
pub type FILT_R = crate::BitReader<bool>;
#[doc = "Field `FILT` writer - Enable Digital Pulse Width Filter"]
pub type FILT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `RSTEN` reader - Enable PCNT Clock Domain Reset"]
pub type RSTEN_R = crate::BitReader<bool>;
#[doc = "Field `RSTEN` writer - Enable PCNT Clock Domain Reset"]
pub type RSTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `CNTRSTEN` reader - Enable CNT Reset"]
pub type CNTRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `CNTRSTEN` writer - Enable CNT Reset"]
pub type CNTRSTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `DEBUGHALT` reader - Debug Mode Halt Enable"]
pub type DEBUGHALT_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGHALT` writer - Debug Mode Halt Enable"]
pub type DEBUGHALT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HYST_R = crate::BitReader<bool>;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HYST_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `S1CDIR` reader - Count Direction Determined By S1"]
pub type S1CDIR_R = crate::BitReader<bool>;
#[doc = "Field `S1CDIR` writer - Count Direction Determined By S1"]
pub type S1CDIR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTEV_A {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    BOTH = 0,
    #[doc = "1: Only counts up on up-count events."]
    UP = 1,
    #[doc = "2: Only counts down on down-count events."]
    DOWN = 2,
    #[doc = "3: Never counts."]
    NONE = 3,
}
impl From<CNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTEV` reader - Controls When the Counter Counts"]
pub type CNTEV_R = crate::FieldReader<u8, CNTEV_A>;
impl CNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEV_A {
        match self.bits {
            0 => CNTEV_A::BOTH,
            1 => CNTEV_A::UP,
            2 => CNTEV_A::DOWN,
            3 => CNTEV_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV_A::BOTH
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV_A::DOWN
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CNTEV_A::NONE
    }
}
#[doc = "Field `CNTEV` writer - Controls When the Counter Counts"]
pub type CNTEV_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CNTEV_A, 2, 10>;
impl<'a> CNTEV_W<'a> {
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CNTEV_A::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CNTEV_A::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(CNTEV_A::DOWN)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CNTEV_A::NONE)
    }
}
#[doc = "Controls When the Auxiliary Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXCNTEV_A {
    #[doc = "0: Never counts."]
    NONE = 0,
    #[doc = "1: Counts up on up-count events."]
    UP = 1,
    #[doc = "2: Counts up on down-count events."]
    DOWN = 2,
    #[doc = "3: Counts up on both up-count and down-count events."]
    BOTH = 3,
}
impl From<AUXCNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCNTEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXCNTEV` reader - Controls When the Auxiliary Counter Counts"]
pub type AUXCNTEV_R = crate::FieldReader<u8, AUXCNTEV_A>;
impl AUXCNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXCNTEV_A {
        match self.bits {
            0 => AUXCNTEV_A::NONE,
            1 => AUXCNTEV_A::UP,
            2 => AUXCNTEV_A::DOWN,
            3 => AUXCNTEV_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV_A::DOWN
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV_A::BOTH
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls When the Auxiliary Counter Counts"]
pub type AUXCNTEV_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, AUXCNTEV_A, 2, 12>;
impl<'a> AUXCNTEV_W<'a> {
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::NONE)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::DOWN)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(AUXCNTEV_A::BOTH)
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_R = crate::BitReader<bool>;
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 14>;
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EDGE_R = crate::BitReader<bool>;
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EDGE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 15>;
#[doc = "Sets the Mode for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCMODE_A {
    #[doc = "0: Triggered compare and clear not enabled."]
    DISABLED = 0,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    LFA = 1,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
    PRS = 2,
}
impl From<TCCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCCMODE` reader - Sets the Mode for Triggered Compare and Clear"]
pub type TCCMODE_R = crate::FieldReader<u8, TCCMODE_A>;
impl TCCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCCMODE_A> {
        match self.bits {
            0 => Some(TCCMODE_A::DISABLED),
            1 => Some(TCCMODE_A::LFA),
            2 => Some(TCCMODE_A::PRS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFA`"]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODE_A::LFA
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODE_A::PRS
    }
}
#[doc = "Field `TCCMODE` writer - Sets the Mode for Triggered Compare and Clear"]
pub type TCCMODE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, TCCMODE_A, 2, 16>;
impl<'a> TCCMODE_W<'a> {
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCCMODE_A::DISABLED)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut W {
        self.variant(TCCMODE_A::LFA)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(TCCMODE_A::PRS)
    }
}
#[doc = "Set the LFA Prescaler for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCPRESC_A {
    #[doc = "0: Compare and clear event each LFA cycle."]
    DIV1 = 0,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    DIV2 = 1,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    DIV4 = 2,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
    DIV8 = 3,
}
impl From<TCCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCCPRESC` reader - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TCCPRESC_R = crate::FieldReader<u8, TCCPRESC_A>;
impl TCCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCPRESC_A {
        match self.bits {
            0 => TCCPRESC_A::DIV1,
            1 => TCCPRESC_A::DIV2,
            2 => TCCPRESC_A::DIV4,
            3 => TCCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESC_A::DIV8
    }
}
#[doc = "Field `TCCPRESC` writer - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TCCPRESC_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, TCCPRESC_A, 2, 19>;
impl<'a> TCCPRESC_W<'a> {
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(TCCPRESC_A::DIV8)
    }
}
#[doc = "Triggered Compare and Clear Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCCOMP_A {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    LTOE = 0,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    GTOE = 1,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    RANGE = 2,
}
impl From<TCCCOMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCCOMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCCCOMP` reader - Triggered Compare and Clear Compare Mode"]
pub type TCCCOMP_R = crate::FieldReader<u8, TCCCOMP_A>;
impl TCCCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCCCOMP_A> {
        match self.bits {
            0 => Some(TCCCOMP_A::LTOE),
            1 => Some(TCCCOMP_A::GTOE),
            2 => Some(TCCCOMP_A::RANGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LTOE`"]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMP_A::LTOE
    }
    #[doc = "Checks if the value of the field is `GTOE`"]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMP_A::GTOE
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMP_A::RANGE
    }
}
#[doc = "Field `TCCCOMP` writer - Triggered Compare and Clear Compare Mode"]
pub type TCCCOMP_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, TCCCOMP_A, 2, 22>;
impl<'a> TCCCOMP_W<'a> {
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut W {
        self.variant(TCCCOMP_A::LTOE)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut W {
        self.variant(TCCCOMP_A::GTOE)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut W {
        self.variant(TCCCOMP_A::RANGE)
    }
}
#[doc = "Field `PRSGATEEN` reader - PRS Gate Enable"]
pub type PRSGATEEN_R = crate::BitReader<bool>;
#[doc = "Field `PRSGATEEN` writer - PRS Gate Enable"]
pub type PRSGATEEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 24>;
#[doc = "Field `TCCPRSPOL` reader - TCC PRS Polarity Select"]
pub type TCCPRSPOL_R = crate::BitReader<bool>;
#[doc = "Field `TCCPRSPOL` writer - TCC PRS Polarity Select"]
pub type TCCPRSPOL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 25>;
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
}
impl From<TCCPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCCPRSSEL` reader - TCC PRS Channel Select"]
pub type TCCPRSSEL_R = crate::FieldReader<u8, TCCPRSSEL_A>;
impl TCCPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCPRSSEL_A {
        match self.bits {
            0 => TCCPRSSEL_A::PRSCH0,
            1 => TCCPRSSEL_A::PRSCH1,
            2 => TCCPRSSEL_A::PRSCH2,
            3 => TCCPRSSEL_A::PRSCH3,
            4 => TCCPRSSEL_A::PRSCH4,
            5 => TCCPRSSEL_A::PRSCH5,
            6 => TCCPRSSEL_A::PRSCH6,
            7 => TCCPRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH7
    }
}
#[doc = "Field `TCCPRSSEL` writer - TCC PRS Channel Select"]
pub type TCCPRSSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, TCCPRSSEL_A, 3, 26>;
impl<'a> TCCPRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(TCCPRSSEL_A::PRSCH7)
    }
}
#[doc = "Field `TOPBHFSEL` reader - TOPB High Frequency Value Select"]
pub type TOPBHFSEL_R = crate::BitReader<bool>;
#[doc = "Field `TOPBHFSEL` writer - TOPB High Frequency Value Select"]
pub type TOPBHFSEL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&self) -> CNTRSTEN_R {
        CNTRSTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AUXCNTRSTEN_R {
        AUXCNTRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DEBUGHALT_R {
        DEBUGHALT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1CDIR_R {
        S1CDIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CNTEV_R {
        CNTEV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AUXCNTEV_R {
        AUXCNTEV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CNTDIR_R {
        CNTDIR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TCCMODE_R {
        TCCMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TCCPRESC_R {
        TCCPRESC_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TCCCOMP_R {
        TCCCOMP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PRSGATEEN_R {
        PRSGATEEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TCCPRSPOL_R {
        TCCPRSPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TCCPRSSEL_R {
        TCCPRSSEL_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&self) -> TOPBHFSEL_R {
        TOPBHFSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W::new(self)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RSTEN_W {
        RSTEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&mut self) -> CNTRSTEN_W {
        CNTRSTEN_W::new(self)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&mut self) -> AUXCNTRSTEN_W {
        AUXCNTRSTEN_W::new(self)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&mut self) -> DEBUGHALT_W {
        DEBUGHALT_W::new(self)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W::new(self)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&mut self) -> S1CDIR_W {
        S1CDIR_W::new(self)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&mut self) -> CNTEV_W {
        CNTEV_W::new(self)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&mut self) -> AUXCNTEV_W {
        AUXCNTEV_W::new(self)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&mut self) -> CNTDIR_W {
        CNTDIR_W::new(self)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W::new(self)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&mut self) -> TCCMODE_W {
        TCCMODE_W::new(self)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&mut self) -> TCCPRESC_W {
        TCCPRESC_W::new(self)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&mut self) -> TCCCOMP_W {
        TCCCOMP_W::new(self)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&mut self) -> PRSGATEEN_W {
        PRSGATEEN_W::new(self)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&mut self) -> TCCPRSPOL_W {
        TCCPRSPOL_W::new(self)
    }
    #[doc = "Bits 26:28 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&mut self) -> TCCPRSSEL_W {
        TCCPRSSEL_W::new(self)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&mut self) -> TOPBHFSEL_W {
        TOPBHFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
