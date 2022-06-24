#[doc = "Register `HFPERPRESC` reader"]
pub struct R(crate::R<HFPERPRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPERPRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPERPRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPERPRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFPERPRESC` writer"]
pub struct W(crate::W<HFPERPRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPERPRESC_SPEC>;
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
impl From<crate::W<HFPERPRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPERPRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFPERCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESC` reader - HFPERCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<u16, PRESC_A>;
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
#[doc = "Field `PRESC` writer - HFPERCLK Prescaler"]
pub type PRESC_W<'a> = crate::FieldWriter<'a, u32, HFPERPRESC_SPEC, u16, PRESC_A, 9, 8>;
impl<'a> PRESC_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
impl R {
    #[doc = "Bits 8:16 - HFPERCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:16 - HFPERCLK Prescaler"]
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
#[doc = "High Frequency Peripheral Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperpresc](index.html) module"]
pub struct HFPERPRESC_SPEC;
impl crate::RegisterSpec for HFPERPRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfperpresc::R](R) reader structure"]
impl crate::Readable for HFPERPRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfperpresc::W](W) writer structure"]
impl crate::Writable for HFPERPRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFPERPRESC to value 0"]
impl crate::Resettable for HFPERPRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
