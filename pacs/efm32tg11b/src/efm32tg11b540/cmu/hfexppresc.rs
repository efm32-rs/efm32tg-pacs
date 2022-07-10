#[doc = "Register `HFEXPPRESC` reader"]
pub struct R(crate::R<HFEXPPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFEXPPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFEXPPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFEXPPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFEXPPRESC` writer"]
pub struct W(crate::W<HFEXPPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFEXPPRESC_SPEC>;
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
impl From<crate::W<HFEXPPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFEXPPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFEXPCLK Prescaler\n\nValue on reset: 0"]
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
#[doc = "Field `PRESC` reader - HFEXPCLK Prescaler"]
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
#[doc = "Field `PRESC` writer - HFEXPCLK Prescaler"]
pub type PRESC_W<'a> = crate::FieldWriter<'a, u32, HFEXPPRESC_SPEC, u8, PRESC_A, 5, 8>;
impl<'a> PRESC_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFEXPCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFEXPCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Export Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfexppresc](index.html) module"]
pub struct HFEXPPRESC_SPEC;
impl crate::RegisterSpec for HFEXPPRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfexppresc::R](R) reader structure"]
impl crate::Readable for HFEXPPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfexppresc::W](W) writer structure"]
impl crate::Writable for HFEXPPRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFEXPPRESC to value 0"]
impl crate::Resettable for HFEXPPRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
