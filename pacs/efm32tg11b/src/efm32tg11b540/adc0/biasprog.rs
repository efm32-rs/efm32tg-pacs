#[doc = "Register `BIASPROG` reader"]
pub struct R(crate::R<BIASPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASPROG` writer"]
pub struct W(crate::W<BIASPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASPROG_SPEC>;
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
impl From<crate::W<BIASPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASPROG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bias Programming Value of Analog ADC Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCBIASPROG_A {
    #[doc = "0: Normal power (use for 1Msps operation)"]
    NORMAL = 0,
    #[doc = "4: Scaling bias to 1/2"]
    SCALE2 = 4,
    #[doc = "8: Scaling bias to 1/4"]
    SCALE4 = 8,
    #[doc = "12: Scaling bias to 1/8"]
    SCALE8 = 12,
    #[doc = "14: Scaling bias to 1/16"]
    SCALE16 = 14,
    #[doc = "15: Scaling bias to 1/32"]
    SCALE32 = 15,
}
impl From<ADCBIASPROG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCBIASPROG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCBIASPROG` reader - Bias Programming Value of Analog ADC Block"]
pub type ADCBIASPROG_R = crate::FieldReader<u8, ADCBIASPROG_A>;
impl ADCBIASPROG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCBIASPROG_A> {
        match self.bits {
            0 => Some(ADCBIASPROG_A::NORMAL),
            4 => Some(ADCBIASPROG_A::SCALE2),
            8 => Some(ADCBIASPROG_A::SCALE4),
            12 => Some(ADCBIASPROG_A::SCALE8),
            14 => Some(ADCBIASPROG_A::SCALE16),
            15 => Some(ADCBIASPROG_A::SCALE32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ADCBIASPROG_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SCALE2`"]
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE2
    }
    #[doc = "Checks if the value of the field is `SCALE4`"]
    #[inline(always)]
    pub fn is_scale4(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE4
    }
    #[doc = "Checks if the value of the field is `SCALE8`"]
    #[inline(always)]
    pub fn is_scale8(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE8
    }
    #[doc = "Checks if the value of the field is `SCALE16`"]
    #[inline(always)]
    pub fn is_scale16(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE16
    }
    #[doc = "Checks if the value of the field is `SCALE32`"]
    #[inline(always)]
    pub fn is_scale32(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE32
    }
}
#[doc = "Field `ADCBIASPROG` writer - Bias Programming Value of Analog ADC Block"]
pub type ADCBIASPROG_W<'a> = crate::FieldWriter<'a, u32, BIASPROG_SPEC, u8, ADCBIASPROG_A, 4, 0>;
impl<'a> ADCBIASPROG_W<'a> {
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::NORMAL)
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline(always)]
    pub fn scale2(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE2)
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline(always)]
    pub fn scale4(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE4)
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline(always)]
    pub fn scale8(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE8)
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline(always)]
    pub fn scale16(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE16)
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline(always)]
    pub fn scale32(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE32)
    }
}
#[doc = "Field `VFAULTCLR` reader - Clear VREFOF Flag"]
pub type VFAULTCLR_R = crate::BitReader<bool>;
#[doc = "Field `VFAULTCLR` writer - Clear VREFOF Flag"]
pub type VFAULTCLR_W<'a> = crate::BitWriter<'a, u32, BIASPROG_SPEC, bool, 12>;
#[doc = "Field `GPBIASACC` reader - Accuracy Setting for the System Bias During ADC Operation"]
pub type GPBIASACC_R = crate::BitReader<bool>;
#[doc = "Field `GPBIASACC` writer - Accuracy Setting for the System Bias During ADC Operation"]
pub type GPBIASACC_W<'a> = crate::BitWriter<'a, u32, BIASPROG_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    pub fn adcbiasprog(&self) -> ADCBIASPROG_R {
        ADCBIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    pub fn vfaultclr(&self) -> VFAULTCLR_R {
        VFAULTCLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    pub fn gpbiasacc(&self) -> GPBIASACC_R {
        GPBIASACC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    pub fn adcbiasprog(&mut self) -> ADCBIASPROG_W {
        ADCBIASPROG_W::new(self)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    pub fn vfaultclr(&mut self) -> VFAULTCLR_W {
        VFAULTCLR_W::new(self)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    pub fn gpbiasacc(&mut self) -> GPBIASACC_W {
        GPBIASACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasprog](index.html) module"]
pub struct BIASPROG_SPEC;
impl crate::RegisterSpec for BIASPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasprog::R](R) reader structure"]
impl crate::Readable for BIASPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasprog::W](W) writer structure"]
impl crate::Writable for BIASPROG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIASPROG to value 0"]
impl crate::Resettable for BIASPROG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
