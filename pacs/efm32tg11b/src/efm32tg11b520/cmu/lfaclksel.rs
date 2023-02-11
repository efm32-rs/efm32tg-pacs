#[doc = "Register `LFACLKSEL` reader"]
pub struct R(crate::R<LFACLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFACLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFACLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFACLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFACLKSEL` writer"]
pub struct W(crate::W<LFACLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFACLKSEL_SPEC>;
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
impl From<crate::W<LFACLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFACLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LFA_R = crate::FieldReader<u8, LFA_A>;
#[doc = "Clock Select for LFA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFA_A {
    #[doc = "0: LFACLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFACLK"]
    ULFRCO = 4,
}
impl From<LFA_A> for u8 {
    #[inline(always)]
    fn from(variant: LFA_A) -> Self {
        variant as _
    }
}
impl LFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFA_A> {
        match self.bits {
            0 => Some(LFA_A::DISABLED),
            1 => Some(LFA_A::LFRCO),
            2 => Some(LFA_A::LFXO),
            4 => Some(LFA_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFA_A::ULFRCO
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LFA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFACLKSEL_SPEC, u8, LFA_A, 3, O>;
impl<'a, const O: u8> LFA_W<'a, O> {
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFA_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFA_A::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFA_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFA_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LFA_R {
        LFA_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    #[must_use]
    pub fn lfa(&mut self) -> LFA_W<0> {
        LFA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency A Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfaclksel](index.html) module"]
pub struct LFACLKSEL_SPEC;
impl crate::RegisterSpec for LFACLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfaclksel::R](R) reader structure"]
impl crate::Readable for LFACLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfaclksel::W](W) writer structure"]
impl crate::Writable for LFACLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFACLKSEL to value 0"]
impl crate::Resettable for LFACLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
