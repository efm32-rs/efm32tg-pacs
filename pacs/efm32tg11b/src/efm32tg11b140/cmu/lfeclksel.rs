#[doc = "Register `LFECLKSEL` reader"]
pub struct R(crate::R<LFECLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFECLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFECLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFECLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFECLKSEL` writer"]
pub struct W(crate::W<LFECLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFECLKSEL_SPEC>;
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
impl From<crate::W<LFECLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFECLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFE` reader - Clock Select for LFE"]
pub type LFE_R = crate::FieldReader<u8, LFE_A>;
#[doc = "Clock Select for LFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFE_A {
    #[doc = "0: LFECLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFECLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFECLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFECLK"]
    ULFRCO = 4,
}
impl From<LFE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFE_A) -> Self {
        variant as _
    }
}
impl LFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFE_A> {
        match self.bits {
            0 => Some(LFE_A::DISABLED),
            1 => Some(LFE_A::LFRCO),
            2 => Some(LFE_A::LFXO),
            4 => Some(LFE_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFE_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFE_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFE_A::ULFRCO
    }
}
#[doc = "Field `LFE` writer - Clock Select for LFE"]
pub type LFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFECLKSEL_SPEC, u8, LFE_A, 3, O>;
impl<'a, const O: u8> LFE_W<'a, O> {
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFE_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFE_A::LFRCO)
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFE_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFE_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&self) -> LFE_R {
        LFE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    #[must_use]
    pub fn lfe(&mut self) -> LFE_W<0> {
        LFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency E Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfeclksel](index.html) module"]
pub struct LFECLKSEL_SPEC;
impl crate::RegisterSpec for LFECLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfeclksel::R](R) reader structure"]
impl crate::Readable for LFECLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfeclksel::W](W) writer structure"]
impl crate::Writable for LFECLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFECLKSEL to value 0"]
impl crate::Resettable for LFECLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
