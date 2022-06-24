#[doc = "Register `HFXOCTRL1` reader"]
pub struct R(crate::R<HFXOCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOCTRL1` writer"]
pub struct W(crate::W<HFXOCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOCTRL1_SPEC>;
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
impl From<crate::W<HFXOCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sets the Amplitude Detection Level (mV)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETTHR_A {
    #[doc = "0: 50mV amplitude detection level"]
    THR0 = 0,
    #[doc = "1: 75mV amplitude detection level"]
    THR1 = 1,
    #[doc = "2: 115mV amplitude detection level"]
    THR2 = 2,
    #[doc = "3: 160mV amplitude detection level"]
    THR3 = 3,
    #[doc = "4: 220mV amplitude detection level"]
    THR4 = 4,
    #[doc = "5: 260mV amplitude detection level"]
    THR5 = 5,
    #[doc = "6: 320mV amplitude detection level"]
    THR6 = 6,
    #[doc = "7: Same as THR6"]
    THR7 = 7,
}
impl From<PEAKDETTHR_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETTHR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEAKDETTHR` reader - Sets the Amplitude Detection Level (mV)"]
pub type PEAKDETTHR_R = crate::FieldReader<u8, PEAKDETTHR_A>;
impl PEAKDETTHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEAKDETTHR_A {
        match self.bits {
            0 => PEAKDETTHR_A::THR0,
            1 => PEAKDETTHR_A::THR1,
            2 => PEAKDETTHR_A::THR2,
            3 => PEAKDETTHR_A::THR3,
            4 => PEAKDETTHR_A::THR4,
            5 => PEAKDETTHR_A::THR5,
            6 => PEAKDETTHR_A::THR6,
            7 => PEAKDETTHR_A::THR7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THR0`"]
    #[inline(always)]
    pub fn is_thr0(&self) -> bool {
        *self == PEAKDETTHR_A::THR0
    }
    #[doc = "Checks if the value of the field is `THR1`"]
    #[inline(always)]
    pub fn is_thr1(&self) -> bool {
        *self == PEAKDETTHR_A::THR1
    }
    #[doc = "Checks if the value of the field is `THR2`"]
    #[inline(always)]
    pub fn is_thr2(&self) -> bool {
        *self == PEAKDETTHR_A::THR2
    }
    #[doc = "Checks if the value of the field is `THR3`"]
    #[inline(always)]
    pub fn is_thr3(&self) -> bool {
        *self == PEAKDETTHR_A::THR3
    }
    #[doc = "Checks if the value of the field is `THR4`"]
    #[inline(always)]
    pub fn is_thr4(&self) -> bool {
        *self == PEAKDETTHR_A::THR4
    }
    #[doc = "Checks if the value of the field is `THR5`"]
    #[inline(always)]
    pub fn is_thr5(&self) -> bool {
        *self == PEAKDETTHR_A::THR5
    }
    #[doc = "Checks if the value of the field is `THR6`"]
    #[inline(always)]
    pub fn is_thr6(&self) -> bool {
        *self == PEAKDETTHR_A::THR6
    }
    #[doc = "Checks if the value of the field is `THR7`"]
    #[inline(always)]
    pub fn is_thr7(&self) -> bool {
        *self == PEAKDETTHR_A::THR7
    }
}
#[doc = "Field `PEAKDETTHR` writer - Sets the Amplitude Detection Level (mV)"]
pub type PEAKDETTHR_W<'a> =
    crate::FieldWriterSafe<'a, u32, HFXOCTRL1_SPEC, u8, PEAKDETTHR_A, 3, 12>;
impl<'a> PEAKDETTHR_W<'a> {
    #[doc = "50mV amplitude detection level"]
    #[inline(always)]
    pub fn thr0(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR0)
    }
    #[doc = "75mV amplitude detection level"]
    #[inline(always)]
    pub fn thr1(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR1)
    }
    #[doc = "115mV amplitude detection level"]
    #[inline(always)]
    pub fn thr2(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR2)
    }
    #[doc = "160mV amplitude detection level"]
    #[inline(always)]
    pub fn thr3(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR3)
    }
    #[doc = "220mV amplitude detection level"]
    #[inline(always)]
    pub fn thr4(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR4)
    }
    #[doc = "260mV amplitude detection level"]
    #[inline(always)]
    pub fn thr5(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR5)
    }
    #[doc = "320mV amplitude detection level"]
    #[inline(always)]
    pub fn thr6(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR6)
    }
    #[doc = "Same as THR6"]
    #[inline(always)]
    pub fn thr7(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR7)
    }
}
impl R {
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PEAKDETTHR_R {
        PEAKDETTHR_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline(always)]
    pub fn peakdetthr(&mut self) -> PEAKDETTHR_W {
        PEAKDETTHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxoctrl1](index.html) module"]
pub struct HFXOCTRL1_SPEC;
impl crate::RegisterSpec for HFXOCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxoctrl1::R](R) reader structure"]
impl crate::Readable for HFXOCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxoctrl1::W](W) writer structure"]
impl crate::Writable for HFXOCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOCTRL1 to value 0x2000"]
impl crate::Resettable for HFXOCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
