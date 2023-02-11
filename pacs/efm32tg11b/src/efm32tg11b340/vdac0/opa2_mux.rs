#[doc = "Register `OPA2_MUX` reader"]
pub struct R(crate::R<OPA2_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA2_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA2_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA2_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA2_MUX` writer"]
pub struct W(crate::W<OPA2_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA2_MUX_SPEC>;
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
impl From<crate::W<OPA2_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA2_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSSEL` reader - OPAx Non-inverting Input Mux"]
pub type POSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSSEL` writer - OPAx Non-inverting Input Mux"]
pub type POSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_MUX_SPEC, u8, u8, 8, O>;
#[doc = "Field `NEGSEL` reader - OPAx Inverting Input Mux"]
pub type NEGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEGSEL` writer - OPAx Inverting Input Mux"]
pub type NEGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_MUX_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESINMUX` reader - OPAx Resistor Ladder Input Mux"]
pub type RESINMUX_R = crate::FieldReader<u8, RESINMUX_A>;
#[doc = "OPAx Resistor Ladder Input Mux\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESINMUX_A {
    #[doc = "0: Set for Unity Gain"]
    DISABLE = 0,
    #[doc = "1: Set for NEXTOUT(x-1) input"]
    OPANEXT = 1,
    #[doc = "2: NEG pad connected"]
    NEGPAD = 2,
    #[doc = "3: POS pad connected"]
    POSPAD = 3,
    #[doc = "4: Neg pad of OPA0 connected. Direct input to support common reference."]
    COMPAD = 4,
    #[doc = "5: OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    CENTER = 5,
    #[doc = "6: VSS connected"]
    VSS = 6,
}
impl From<RESINMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: RESINMUX_A) -> Self {
        variant as _
    }
}
impl RESINMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESINMUX_A> {
        match self.bits {
            0 => Some(RESINMUX_A::DISABLE),
            1 => Some(RESINMUX_A::OPANEXT),
            2 => Some(RESINMUX_A::NEGPAD),
            3 => Some(RESINMUX_A::POSPAD),
            4 => Some(RESINMUX_A::COMPAD),
            5 => Some(RESINMUX_A::CENTER),
            6 => Some(RESINMUX_A::VSS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESINMUX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `OPANEXT`"]
    #[inline(always)]
    pub fn is_opanext(&self) -> bool {
        *self == RESINMUX_A::OPANEXT
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline(always)]
    pub fn is_negpad(&self) -> bool {
        *self == RESINMUX_A::NEGPAD
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline(always)]
    pub fn is_pospad(&self) -> bool {
        *self == RESINMUX_A::POSPAD
    }
    #[doc = "Checks if the value of the field is `COMPAD`"]
    #[inline(always)]
    pub fn is_compad(&self) -> bool {
        *self == RESINMUX_A::COMPAD
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == RESINMUX_A::CENTER
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == RESINMUX_A::VSS
    }
}
#[doc = "Field `RESINMUX` writer - OPAx Resistor Ladder Input Mux"]
pub type RESINMUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPA2_MUX_SPEC, u8, RESINMUX_A, 3, O>;
impl<'a, const O: u8> RESINMUX_W<'a, O> {
    #[doc = "Set for Unity Gain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESINMUX_A::DISABLE)
    }
    #[doc = "Set for NEXTOUT(x-1) input"]
    #[inline(always)]
    pub fn opanext(self) -> &'a mut W {
        self.variant(RESINMUX_A::OPANEXT)
    }
    #[doc = "NEG pad connected"]
    #[inline(always)]
    pub fn negpad(self) -> &'a mut W {
        self.variant(RESINMUX_A::NEGPAD)
    }
    #[doc = "POS pad connected"]
    #[inline(always)]
    pub fn pospad(self) -> &'a mut W {
        self.variant(RESINMUX_A::POSPAD)
    }
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    #[inline(always)]
    pub fn compad(self) -> &'a mut W {
        self.variant(RESINMUX_A::COMPAD)
    }
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(RESINMUX_A::CENTER)
    }
    #[doc = "VSS connected"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(RESINMUX_A::VSS)
    }
}
#[doc = "Field `GAIN3X` reader - OPAx Dedicated 3x Gain Resistor Ladder"]
pub type GAIN3X_R = crate::BitReader<bool>;
#[doc = "Field `GAIN3X` writer - OPAx Dedicated 3x Gain Resistor Ladder"]
pub type GAIN3X_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPA2_MUX_SPEC, bool, O>;
#[doc = "Field `RESSEL` reader - OPAx Resistor Ladder Select"]
pub type RESSEL_R = crate::FieldReader<u8, RESSEL_A>;
#[doc = "OPAx Resistor Ladder Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESSEL_A {
    #[doc = "0: Gain of 1/3"]
    RES0 = 0,
    #[doc = "1: Gain of 1"]
    RES1 = 1,
    #[doc = "2: Gain of 1 2/3"]
    RES2 = 2,
    #[doc = "3: Gain of 2 1/5"]
    RES3 = 3,
    #[doc = "4: Gain of 3"]
    RES4 = 4,
    #[doc = "5: Gain of 4 1/3"]
    RES5 = 5,
    #[doc = "6: Gain of 7"]
    RES6 = 6,
    #[doc = "7: Gain of 15"]
    RES7 = 7,
}
impl From<RESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSEL_A) -> Self {
        variant as _
    }
}
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSEL_A {
        match self.bits {
            0 => RESSEL_A::RES0,
            1 => RESSEL_A::RES1,
            2 => RESSEL_A::RES2,
            3 => RESSEL_A::RES3,
            4 => RESSEL_A::RES4,
            5 => RESSEL_A::RES5,
            6 => RESSEL_A::RES6,
            7 => RESSEL_A::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == RESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == RESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == RESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == RESSEL_A::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == RESSEL_A::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == RESSEL_A::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == RESSEL_A::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == RESSEL_A::RES7
    }
}
#[doc = "Field `RESSEL` writer - OPAx Resistor Ladder Select"]
pub type RESSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPA2_MUX_SPEC, u8, RESSEL_A, 3, O>;
impl<'a, const O: u8> RESSEL_W<'a, O> {
    #[doc = "Gain of 1/3"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(RESSEL_A::RES0)
    }
    #[doc = "Gain of 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(RESSEL_A::RES1)
    }
    #[doc = "Gain of 1 2/3"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(RESSEL_A::RES2)
    }
    #[doc = "Gain of 2 1/5"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(RESSEL_A::RES3)
    }
    #[doc = "Gain of 3"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut W {
        self.variant(RESSEL_A::RES4)
    }
    #[doc = "Gain of 4 1/3"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut W {
        self.variant(RESSEL_A::RES5)
    }
    #[doc = "Gain of 7"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut W {
        self.variant(RESSEL_A::RES6)
    }
    #[doc = "Gain of 15"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut W {
        self.variant(RESSEL_A::RES7)
    }
}
impl R {
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline(always)]
    pub fn resinmux(&self) -> RESINMUX_R {
        RESINMUX_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline(always)]
    pub fn gain3x(&self) -> GAIN3X_R {
        GAIN3X_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> POSSEL_W<0> {
        POSSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NEGSEL_W<8> {
        NEGSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline(always)]
    #[must_use]
    pub fn resinmux(&mut self) -> RESINMUX_W<16> {
        RESINMUX_W::new(self)
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline(always)]
    #[must_use]
    pub fn gain3x(&mut self) -> GAIN3X_W<20> {
        GAIN3X_W::new(self)
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<24> {
        RESSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Mux Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_mux](index.html) module"]
pub struct OPA2_MUX_SPEC;
impl crate::RegisterSpec for OPA2_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa2_mux::R](R) reader structure"]
impl crate::Readable for OPA2_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa2_mux::W](W) writer structure"]
impl crate::Writable for OPA2_MUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA2_MUX to value 0x0016_f2f1"]
impl crate::Resettable for OPA2_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0x0016_f2f1;
}
