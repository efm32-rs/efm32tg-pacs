#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SINGLEACT` reader - Single Channel Conversion Active"]
pub type SINGLEACT_R = crate::BitReader<bool>;
#[doc = "Field `SCANACT` reader - Scan Conversion Active"]
pub type SCANACT_R = crate::BitReader<bool>;
#[doc = "Field `SCANPENDING` reader - Scan Conversion Pending"]
pub type SCANPENDING_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEREFWARM` reader - Single Channel Reference Warmed Up"]
pub type SINGLEREFWARM_R = crate::BitReader<bool>;
#[doc = "Field `SCANREFWARM` reader - Scan Reference Warmed Up"]
pub type SCANREFWARM_R = crate::BitReader<bool>;
#[doc = "Field `PROGERR` reader - Programming Error Status"]
pub type PROGERR_R = crate::FieldReader<u8, PROGERR_A>;
#[doc = "Programming Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROGERR_A {
    #[doc = "1: `1`"]
    BUSCONF = 1,
    #[doc = "2: `10`"]
    NEGSELCONF = 2,
}
impl From<PROGERR_A> for u8 {
    #[inline(always)]
    fn from(variant: PROGERR_A) -> Self {
        variant as _
    }
}
impl PROGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROGERR_A> {
        match self.bits {
            1 => Some(PROGERR_A::BUSCONF),
            2 => Some(PROGERR_A::NEGSELCONF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUSCONF`"]
    #[inline(always)]
    pub fn is_busconf(&self) -> bool {
        *self == PROGERR_A::BUSCONF
    }
    #[doc = "Checks if the value of the field is `NEGSELCONF`"]
    #[inline(always)]
    pub fn is_negselconf(&self) -> bool {
        *self == PROGERR_A::NEGSELCONF
    }
}
#[doc = "Field `WARM` reader - ADC Warmed Up"]
pub type WARM_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEDV` reader - Single Channel Data Valid"]
pub type SINGLEDV_R = crate::BitReader<bool>;
#[doc = "Field `SCANDV` reader - Scan Data Valid"]
pub type SCANDV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Single Channel Conversion Active"]
    #[inline(always)]
    pub fn singleact(&self) -> SINGLEACT_R {
        SINGLEACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline(always)]
    pub fn scanact(&self) -> SCANACT_R {
        SCANACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Conversion Pending"]
    #[inline(always)]
    pub fn scanpending(&self) -> SCANPENDING_R {
        SCANPENDING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Channel Reference Warmed Up"]
    #[inline(always)]
    pub fn singlerefwarm(&self) -> SINGLEREFWARM_R {
        SINGLEREFWARM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline(always)]
    pub fn scanrefwarm(&self) -> SCANREFWARM_R {
        SCANREFWARM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Programming Error Status"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline(always)]
    pub fn warm(&self) -> WARM_R {
        WARM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Channel Data Valid"]
    #[inline(always)]
    pub fn singledv(&self) -> SINGLEDV_R {
        SINGLEDV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline(always)]
    pub fn scandv(&self) -> SCANDV_R {
        SCANDV_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
