#[doc = "Register `LFBCLKSEL` reader"]
pub struct R(crate::R<LFBCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFBCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFBCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFBCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFBCLKSEL` writer"]
pub struct W(crate::W<LFBCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFBCLKSEL_SPEC>;
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
impl From<crate::W<LFBCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFBCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Select for LFB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFB_A {
    #[doc = "0: LFBCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    LFXO = 2,
    #[doc = "3: HFCLK divided by two/four is selected as LFBCLK"]
    HFCLKLE = 3,
    #[doc = "4: ULFRCO selected as LFBCLK"]
    ULFRCO = 4,
}
impl From<LFB_A> for u8 {
    #[inline(always)]
    fn from(variant: LFB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LFB_R = crate::FieldReader<u8, LFB_A>;
impl LFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFB_A> {
        match self.bits {
            0 => Some(LFB_A::DISABLED),
            1 => Some(LFB_A::LFRCO),
            2 => Some(LFB_A::LFXO),
            3 => Some(LFB_A::HFCLKLE),
            4 => Some(LFB_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCLKLE`"]
    #[inline(always)]
    pub fn is_hfclkle(&self) -> bool {
        *self == LFB_A::HFCLKLE
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFB_A::ULFRCO
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LFB_W<'a> = crate::FieldWriter<'a, u32, LFBCLKSEL_SPEC, u8, LFB_A, 3, 0>;
impl<'a> LFB_W<'a> {
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFB_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFB_A::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFB_A::LFXO)
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn hfclkle(self) -> &'a mut W {
        self.variant(LFB_A::HFCLKLE)
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFB_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LFB_R {
        LFB_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&mut self) -> LFB_W {
        LFB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency B Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfbclksel](index.html) module"]
pub struct LFBCLKSEL_SPEC;
impl crate::RegisterSpec for LFBCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfbclksel::R](R) reader structure"]
impl crate::Readable for LFBCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfbclksel::W](W) writer structure"]
impl crate::Writable for LFBCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFBCLKSEL to value 0"]
impl crate::Resettable for LFBCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
