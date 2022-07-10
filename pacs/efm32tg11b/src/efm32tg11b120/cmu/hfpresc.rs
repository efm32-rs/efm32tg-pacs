#[doc = "Register `HFPRESC` reader"]
pub struct R(crate::R<HFPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFPRESC` writer"]
pub struct W(crate::W<HFPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPRESC_SPEC>;
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
impl From<crate::W<HFPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - HFCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - HFCLK Prescaler"]
pub type PRESC_W<'a> = crate::FieldWriter<'a, u32, HFPRESC_SPEC, u8, PRESC_A, 5, 8>;
impl<'a> PRESC_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "HFCLKLE Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFCLKLEPRESC_A {
    #[doc = "0: HFCLKLE is HFBUSCLKLE divided by 2."]
    DIV2 = 0,
    #[doc = "1: HFCLKLE is HFBUSCLKLE divided by 4."]
    DIV4 = 1,
    #[doc = "2: HFCLKLE is HFBUSCLKLE divided by 8."]
    DIV8 = 2,
}
impl From<HFCLKLEPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: HFCLKLEPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFCLKLEPRESC` reader - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_R = crate::FieldReader<u8, HFCLKLEPRESC_A>;
impl HFCLKLEPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFCLKLEPRESC_A> {
        match self.bits {
            0 => Some(HFCLKLEPRESC_A::DIV2),
            1 => Some(HFCLKLEPRESC_A::DIV4),
            2 => Some(HFCLKLEPRESC_A::DIV8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HFCLKLEPRESC_A::DIV8
    }
}
#[doc = "Field `HFCLKLEPRESC` writer - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_W<'a> = crate::FieldWriter<'a, u32, HFPRESC_SPEC, u8, HFCLKLEPRESC_A, 2, 24>;
impl<'a> HFCLKLEPRESC_W<'a> {
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV4)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HFCLKLEPRESC_A::DIV8)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HFCLKLEPRESC_R {
        HFCLKLEPRESC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&mut self) -> HFCLKLEPRESC_W {
        HFCLKLEPRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfpresc](index.html) module"]
pub struct HFPRESC_SPEC;
impl crate::RegisterSpec for HFPRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfpresc::R](R) reader structure"]
impl crate::Readable for HFPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfpresc::W](W) writer structure"]
impl crate::Writable for HFPRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFPRESC to value 0"]
impl crate::Resettable for HFPRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
