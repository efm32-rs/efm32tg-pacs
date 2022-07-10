#[doc = "Register `TIMCTRL` reader"]
pub struct R(crate::R<TIMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCTRL` writer"]
pub struct W(crate::W<TIMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCTRL_SPEC>;
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
impl From<crate::W<TIMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Period Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCPRESC_A {
    #[doc = "0: The period counter clock frequency is LFBCLKCSEN/1"]
    DIV1 = 0,
    #[doc = "1: The period counter clock frequency is LFBCLKCSEN/2"]
    DIV2 = 1,
    #[doc = "2: The period counter clock frequency is LFBCLKCSEN/4"]
    DIV4 = 2,
    #[doc = "3: The period counter clock frequency is LFBCLKCSEN/8"]
    DIV8 = 3,
    #[doc = "4: The period counter clock frequency is LFBCLKCSEN/16"]
    DIV16 = 4,
    #[doc = "5: The period counter clock frequency is LFBCLKCSEN/32"]
    DIV32 = 5,
    #[doc = "6: The period counter clock frequency is LFBCLKCSEN/64"]
    DIV64 = 6,
    #[doc = "7: The period counter clock frequency is LFBCLKCSEN/128"]
    DIV128 = 7,
}
impl From<PCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCPRESC` reader - Period Counter Prescaler"]
pub type PCPRESC_R = crate::FieldReader<u8, PCPRESC_A>;
impl PCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPRESC_A {
        match self.bits {
            0 => PCPRESC_A::DIV1,
            1 => PCPRESC_A::DIV2,
            2 => PCPRESC_A::DIV4,
            3 => PCPRESC_A::DIV8,
            4 => PCPRESC_A::DIV16,
            5 => PCPRESC_A::DIV32,
            6 => PCPRESC_A::DIV64,
            7 => PCPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESC_A::DIV128
    }
}
#[doc = "Field `PCPRESC` writer - Period Counter Prescaler"]
pub type PCPRESC_W<'a> = crate::FieldWriterSafe<'a, u32, TIMCTRL_SPEC, u8, PCPRESC_A, 3, 0>;
impl<'a> PCPRESC_W<'a> {
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV1)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV2)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV4)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV8)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV16)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV32)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV64)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV128)
    }
}
#[doc = "Field `PCTOP` reader - Period Counter Top Value"]
pub type PCTOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCTOP` writer - Period Counter Top Value"]
pub type PCTOP_W<'a> = crate::FieldWriter<'a, u32, TIMCTRL_SPEC, u8, u8, 8, 8>;
#[doc = "Field `WARMUPCNT` reader - Warmup Period Counter"]
pub type WARMUPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WARMUPCNT` writer - Warmup Period Counter"]
pub type WARMUPCNT_W<'a> = crate::FieldWriter<'a, u32, TIMCTRL_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PCPRESC_R {
        PCPRESC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&self) -> PCTOP_R {
        PCTOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&self) -> WARMUPCNT_R {
        WARMUPCNT_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&mut self) -> PCPRESC_W {
        PCPRESC_W::new(self)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&mut self) -> PCTOP_W {
        PCTOP_W::new(self)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&mut self) -> WARMUPCNT_W {
        WARMUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctrl](index.html) module"]
pub struct TIMCTRL_SPEC;
impl crate::RegisterSpec for TIMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timctrl::R](R) reader structure"]
impl crate::Readable for TIMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timctrl::W](W) writer structure"]
impl crate::Writable for TIMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMCTRL to value 0"]
impl crate::Resettable for TIMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
