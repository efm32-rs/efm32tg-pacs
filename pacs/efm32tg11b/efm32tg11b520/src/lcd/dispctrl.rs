#[doc = "Register `DISPCTRL` reader"]
pub struct R(crate::R<DISPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISPCTRL` writer"]
pub struct W(crate::W<DISPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPCTRL_SPEC>;
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
impl From<crate::W<DISPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mux Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: Duplex"]
    DUPLEX = 1,
    #[doc = "2: Triplex"]
    TRIPLEX = 2,
    #[doc = "3: Quadruplex"]
    QUADRUPLEX = 3,
    #[doc = "5: Sextaplex"]
    SEXTAPLEX = 5,
    #[doc = "7: Octaplex"]
    OCTAPLEX = 7,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUX` reader - Mux Configuration"]
pub type MUX_R = crate::FieldReader<u8, MUX_A>;
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_A> {
        match self.bits {
            0 => Some(MUX_A::STATIC),
            1 => Some(MUX_A::DUPLEX),
            2 => Some(MUX_A::TRIPLEX),
            3 => Some(MUX_A::QUADRUPLEX),
            5 => Some(MUX_A::SEXTAPLEX),
            7 => Some(MUX_A::OCTAPLEX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == MUX_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DUPLEX`"]
    #[inline(always)]
    pub fn is_duplex(&self) -> bool {
        *self == MUX_A::DUPLEX
    }
    #[doc = "Checks if the value of the field is `TRIPLEX`"]
    #[inline(always)]
    pub fn is_triplex(&self) -> bool {
        *self == MUX_A::TRIPLEX
    }
    #[doc = "Checks if the value of the field is `QUADRUPLEX`"]
    #[inline(always)]
    pub fn is_quadruplex(&self) -> bool {
        *self == MUX_A::QUADRUPLEX
    }
    #[doc = "Checks if the value of the field is `SEXTAPLEX`"]
    #[inline(always)]
    pub fn is_sextaplex(&self) -> bool {
        *self == MUX_A::SEXTAPLEX
    }
    #[doc = "Checks if the value of the field is `OCTAPLEX`"]
    #[inline(always)]
    pub fn is_octaplex(&self) -> bool {
        *self == MUX_A::OCTAPLEX
    }
}
#[doc = "Field `MUX` writer - Mux Configuration"]
pub type MUX_W<'a> = crate::FieldWriter<'a, u32, DISPCTRL_SPEC, u8, MUX_A, 3, 0>;
impl<'a> MUX_W<'a> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(MUX_A::STATIC)
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn duplex(self) -> &'a mut W {
        self.variant(MUX_A::DUPLEX)
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn triplex(self) -> &'a mut W {
        self.variant(MUX_A::TRIPLEX)
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn quadruplex(self) -> &'a mut W {
        self.variant(MUX_A::QUADRUPLEX)
    }
    #[doc = "Sextaplex"]
    #[inline(always)]
    pub fn sextaplex(self) -> &'a mut W {
        self.variant(MUX_A::SEXTAPLEX)
    }
    #[doc = "Octaplex"]
    #[inline(always)]
    pub fn octaplex(self) -> &'a mut W {
        self.variant(MUX_A::OCTAPLEX)
    }
}
#[doc = "Field `WAVE` reader - Waveform Selection"]
pub type WAVE_R = crate::BitReader<bool>;
#[doc = "Field `WAVE` writer - Waveform Selection"]
pub type WAVE_W<'a> = crate::BitWriter<'a, u32, DISPCTRL_SPEC, bool, 4>;
#[doc = "Field `CONTRAST` reader - Contrast Control"]
pub type CONTRAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONTRAST` writer - Contrast Control"]
pub type CONTRAST_W<'a> = crate::FieldWriter<'a, u32, DISPCTRL_SPEC, u8, u8, 6, 8>;
#[doc = "Charge Redistribution Cycles\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHGRDST_A {
    #[doc = "0: Disable charge redistribution."]
    DISABLE = 0,
    #[doc = "1: Use 1 prescaled low frequency clock cycle for charge redistribution."]
    ONE = 1,
    #[doc = "2: Use 2 prescaled low frequency clock cycles for charge redistribution."]
    TWO = 2,
    #[doc = "3: Use 3 prescaled low frequency clock cycles for charge redistribution."]
    THREE = 3,
    #[doc = "4: Use 4 prescaled low frequency clock cycles for charge redistribution."]
    FOUR = 4,
}
impl From<CHGRDST_A> for u8 {
    #[inline(always)]
    fn from(variant: CHGRDST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHGRDST` reader - Charge Redistribution Cycles"]
pub type CHGRDST_R = crate::FieldReader<u8, CHGRDST_A>;
impl CHGRDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHGRDST_A> {
        match self.bits {
            0 => Some(CHGRDST_A::DISABLE),
            1 => Some(CHGRDST_A::ONE),
            2 => Some(CHGRDST_A::TWO),
            3 => Some(CHGRDST_A::THREE),
            4 => Some(CHGRDST_A::FOUR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHGRDST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CHGRDST_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CHGRDST_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CHGRDST_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == CHGRDST_A::FOUR
    }
}
#[doc = "Field `CHGRDST` writer - Charge Redistribution Cycles"]
pub type CHGRDST_W<'a> = crate::FieldWriter<'a, u32, DISPCTRL_SPEC, u8, CHGRDST_A, 3, 20>;
impl<'a> CHGRDST_W<'a> {
    #[doc = "Disable charge redistribution."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHGRDST_A::DISABLE)
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CHGRDST_A::ONE)
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CHGRDST_A::TWO)
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CHGRDST_A::THREE)
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(CHGRDST_A::FOUR)
    }
}
#[doc = "Bias Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_A {
    #[doc = "0: Static"]
    STATIC = 0,
    #[doc = "1: 1/2 Bias"]
    ONEHALF = 1,
    #[doc = "2: 1/3 Bias"]
    ONETHIRD = 2,
    #[doc = "3: 1/4 Bias"]
    ONEFOURTH = 3,
}
impl From<BIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BIAS` reader - Bias Configuration"]
pub type BIAS_R = crate::FieldReader<u8, BIAS_A>;
impl BIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            0 => BIAS_A::STATIC,
            1 => BIAS_A::ONEHALF,
            2 => BIAS_A::ONETHIRD,
            3 => BIAS_A::ONEFOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == BIAS_A::STATIC
    }
    #[doc = "Checks if the value of the field is `ONEHALF`"]
    #[inline(always)]
    pub fn is_onehalf(&self) -> bool {
        *self == BIAS_A::ONEHALF
    }
    #[doc = "Checks if the value of the field is `ONETHIRD`"]
    #[inline(always)]
    pub fn is_onethird(&self) -> bool {
        *self == BIAS_A::ONETHIRD
    }
    #[doc = "Checks if the value of the field is `ONEFOURTH`"]
    #[inline(always)]
    pub fn is_onefourth(&self) -> bool {
        *self == BIAS_A::ONEFOURTH
    }
}
#[doc = "Field `BIAS` writer - Bias Configuration"]
pub type BIAS_W<'a> = crate::FieldWriterSafe<'a, u32, DISPCTRL_SPEC, u8, BIAS_A, 2, 24>;
impl<'a> BIAS_W<'a> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(BIAS_A::STATIC)
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn onehalf(self) -> &'a mut W {
        self.variant(BIAS_A::ONEHALF)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn onethird(self) -> &'a mut W {
        self.variant(BIAS_A::ONETHIRD)
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn onefourth(self) -> &'a mut W {
        self.variant(BIAS_A::ONEFOURTH)
    }
}
#[doc = "Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\]
to control VLCD."]
    NOEXTCAP = 0,
    #[doc = "1: Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\]
to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    STEPDOWN = 1,
    #[doc = "2: Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\]
to control VLCD level, and use SPEED to adjust oscillator frequency."]
    CPINTOSC = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Mode Setting"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::NOEXTCAP),
            1 => Some(MODE_A::STEPDOWN),
            2 => Some(MODE_A::CPINTOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOEXTCAP`"]
    #[inline(always)]
    pub fn is_noextcap(&self) -> bool {
        *self == MODE_A::NOEXTCAP
    }
    #[doc = "Checks if the value of the field is `STEPDOWN`"]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == MODE_A::STEPDOWN
    }
    #[doc = "Checks if the value of the field is `CPINTOSC`"]
    #[inline(always)]
    pub fn is_cpintosc(&self) -> bool {
        *self == MODE_A::CPINTOSC
    }
}
#[doc = "Field `MODE` writer - Mode Setting"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, DISPCTRL_SPEC, u8, MODE_A, 2, 28>;
impl<'a> MODE_W<'a> {
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\]
to control VLCD."]
    #[inline(always)]
    pub fn noextcap(self) -> &'a mut W {
        self.variant(MODE_A::NOEXTCAP)
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\]
to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut W {
        self.variant(MODE_A::STEPDOWN)
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\]
to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline(always)]
    pub fn cpintosc(self) -> &'a mut W {
        self.variant(MODE_A::CPINTOSC)
    }
}
impl R {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&self) -> CONTRAST_R {
        CONTRAST_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&self) -> CHGRDST_R {
        CHGRDST_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W {
        MUX_W::new(self)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W::new(self)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&mut self) -> CONTRAST_W {
        CONTRAST_W::new(self)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&mut self) -> CHGRDST_W {
        CHGRDST_W::new(self)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W::new(self)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dispctrl](index.html) module"]
pub struct DISPCTRL_SPEC;
impl crate::RegisterSpec for DISPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dispctrl::R](R) reader structure"]
impl crate::Readable for DISPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dispctrl::W](W) writer structure"]
impl crate::Writable for DISPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISPCTRL to value 0x0010_3f00"]
impl crate::Resettable for DISPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_3f00
    }
}
