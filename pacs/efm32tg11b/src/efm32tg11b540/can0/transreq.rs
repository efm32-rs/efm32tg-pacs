#[doc = "Register `TRANSREQ` reader"]
pub struct R(crate::R<TRANSREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRQSTOUT` reader - Transmission Request Bits (Of All Message Objects)"]
pub type TXRQSTOUT_R = crate::FieldReader<u32, TXRQSTOUT_A>;
#[doc = "Transmission Request Bits (Of All Message Objects)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TXRQSTOUT_A {
    #[doc = "0: This Message Object is not waiting for transmission."]
    FALSE = 0,
    #[doc = "1: The transmission of this Message Object is requested and is not yet done."]
    TRUE = 1,
}
impl From<TXRQSTOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: TXRQSTOUT_A) -> Self {
        variant as _
    }
}
impl TXRQSTOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXRQSTOUT_A> {
        match self.bits {
            0 => Some(TXRQSTOUT_A::FALSE),
            1 => Some(TXRQSTOUT_A::TRUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == TXRQSTOUT_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == TXRQSTOUT_A::TRUE
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmission Request Bits (Of All Message Objects)"]
    #[inline(always)]
    pub fn txrqstout(&self) -> TXRQSTOUT_R {
        TXRQSTOUT_R::new(self.bits)
    }
}
#[doc = "Transmission Request Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transreq](index.html) module"]
pub struct TRANSREQ_SPEC;
impl crate::RegisterSpec for TRANSREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transreq::R](R) reader structure"]
impl crate::Readable for TRANSREQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRANSREQ to value 0"]
impl crate::Resettable for TRANSREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
