#[doc = "Register `LFEPRESC0` reader"]
pub struct R(crate::R<LFEPRESC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFEPRESC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFEPRESC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFEPRESC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFEPRESC0` writer"]
pub struct W(crate::W<LFEPRESC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFEPRESC0_SPEC>;
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
impl From<crate::W<LFEPRESC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFEPRESC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Real-Time Counter and Calendar Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCC_A {
    #[doc = "0: LFECLKRTCC = LFECLK"]
    DIV1 = 0,
    #[doc = "1: LFECLKRTCC = LFECLK/2"]
    DIV2 = 1,
    #[doc = "2: LFECLKRTCC = LFECLK/4"]
    DIV4 = 2,
}
impl From<RTCC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Prescaler"]
pub type RTCC_R = crate::FieldReader<u8, RTCC_A>;
impl RTCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCC_A> {
        match self.bits {
            0 => Some(RTCC_A::DIV1),
            1 => Some(RTCC_A::DIV2),
            2 => Some(RTCC_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTCC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCC_A::DIV4
    }
}
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar Prescaler"]
pub type RTCC_W<'a> = crate::FieldWriter<'a, u32, LFEPRESC0_SPEC, u8, RTCC_A, 2, 0>;
impl<'a> RTCC_W<'a> {
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTCC_A::DIV1)
    }
    #[doc = "LFECLKRTCC = LFECLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCC_A::DIV2)
    }
    #[doc = "LFECLKRTCC = LFECLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCC_A::DIV4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfepresc0](index.html) module"]
pub struct LFEPRESC0_SPEC;
impl crate::RegisterSpec for LFEPRESC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfepresc0::R](R) reader structure"]
impl crate::Readable for LFEPRESC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfepresc0::W](W) writer structure"]
impl crate::Writable for LFEPRESC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFEPRESC0 to value 0"]
impl crate::Resettable for LFEPRESC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
