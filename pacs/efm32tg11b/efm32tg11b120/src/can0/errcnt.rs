#[doc = "Register `ERRCNT` reader"]
pub struct R(crate::R<ERRCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECERRP` reader - Receive Error Passive"]
pub type RECERRP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn recerrp(&self) -> RECERRP_R {
        RECERRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Error Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errcnt](index.html) module"]
pub struct ERRCNT_SPEC;
impl crate::RegisterSpec for ERRCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errcnt::R](R) reader structure"]
impl crate::Readable for ERRCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERRCNT to value 0"]
impl crate::Resettable for ERRCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
