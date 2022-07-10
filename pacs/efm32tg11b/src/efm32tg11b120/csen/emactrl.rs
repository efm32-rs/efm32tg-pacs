#[doc = "Register `EMACTRL` reader"]
pub struct R(crate::R<EMACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMACTRL` writer"]
pub struct W(crate::W<EMACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMACTRL_SPEC>;
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
impl From<crate::W<EMACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EMA Sample Weight\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMASAMPLE_A {
    #[doc = "0: EMA weight (N) is 1."]
    W1 = 0,
    #[doc = "1: EMA weight (N) is 2."]
    W2 = 1,
    #[doc = "2: EMA weight (N) is 4."]
    W4 = 2,
    #[doc = "3: EMA weight (N) is 8."]
    W8 = 3,
    #[doc = "4: EMA weight (N) is 16."]
    W16 = 4,
    #[doc = "5: EMA weight (N) is 32."]
    W32 = 5,
    #[doc = "6: EMA weight (N) is 64."]
    W64 = 6,
}
impl From<EMASAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMASAMPLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMASAMPLE` reader - EMA Sample Weight"]
pub type EMASAMPLE_R = crate::FieldReader<u8, EMASAMPLE_A>;
impl EMASAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMASAMPLE_A> {
        match self.bits {
            0 => Some(EMASAMPLE_A::W1),
            1 => Some(EMASAMPLE_A::W2),
            2 => Some(EMASAMPLE_A::W4),
            3 => Some(EMASAMPLE_A::W8),
            4 => Some(EMASAMPLE_A::W16),
            5 => Some(EMASAMPLE_A::W32),
            6 => Some(EMASAMPLE_A::W64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `W1`"]
    #[inline(always)]
    pub fn is_w1(&self) -> bool {
        *self == EMASAMPLE_A::W1
    }
    #[doc = "Checks if the value of the field is `W2`"]
    #[inline(always)]
    pub fn is_w2(&self) -> bool {
        *self == EMASAMPLE_A::W2
    }
    #[doc = "Checks if the value of the field is `W4`"]
    #[inline(always)]
    pub fn is_w4(&self) -> bool {
        *self == EMASAMPLE_A::W4
    }
    #[doc = "Checks if the value of the field is `W8`"]
    #[inline(always)]
    pub fn is_w8(&self) -> bool {
        *self == EMASAMPLE_A::W8
    }
    #[doc = "Checks if the value of the field is `W16`"]
    #[inline(always)]
    pub fn is_w16(&self) -> bool {
        *self == EMASAMPLE_A::W16
    }
    #[doc = "Checks if the value of the field is `W32`"]
    #[inline(always)]
    pub fn is_w32(&self) -> bool {
        *self == EMASAMPLE_A::W32
    }
    #[doc = "Checks if the value of the field is `W64`"]
    #[inline(always)]
    pub fn is_w64(&self) -> bool {
        *self == EMASAMPLE_A::W64
    }
}
#[doc = "Field `EMASAMPLE` writer - EMA Sample Weight"]
pub type EMASAMPLE_W<'a> = crate::FieldWriter<'a, u32, EMACTRL_SPEC, u8, EMASAMPLE_A, 3, 0>;
impl<'a> EMASAMPLE_W<'a> {
    #[doc = "EMA weight (N) is 1."]
    #[inline(always)]
    pub fn w1(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W1)
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline(always)]
    pub fn w2(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W2)
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline(always)]
    pub fn w4(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W4)
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline(always)]
    pub fn w8(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W8)
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline(always)]
    pub fn w16(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W16)
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline(always)]
    pub fn w32(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W32)
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline(always)]
    pub fn w64(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W64)
    }
}
impl R {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&self) -> EMASAMPLE_R {
        EMASAMPLE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&mut self) -> EMASAMPLE_W {
        EMASAMPLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Exponential Moving Average Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emactrl](index.html) module"]
pub struct EMACTRL_SPEC;
impl crate::RegisterSpec for EMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emactrl::R](R) reader structure"]
impl crate::Readable for EMACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emactrl::W](W) writer structure"]
impl crate::Writable for EMACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMACTRL to value 0"]
impl crate::Resettable for EMACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
