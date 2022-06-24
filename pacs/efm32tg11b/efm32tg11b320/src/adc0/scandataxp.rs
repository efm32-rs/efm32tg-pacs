#[doc = "Register `SCANDATAXP` reader"]
pub struct R(crate::R<SCANDATAXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANDATAXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANDATAXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANDATAXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DATAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCANINPUTIDPEEK` reader - Scan Conversion Data Source Peek"]
pub type SCANINPUTIDPEEK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Data Source Peek"]
    #[inline(always)]
    pub fn scaninputidpeek(&self) -> SCANINPUTIDPEEK_R {
        SCANINPUTIDPEEK_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Scan Sequence Result Data + Data Source Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandataxp](index.html) module"]
pub struct SCANDATAXP_SPEC;
impl crate::RegisterSpec for SCANDATAXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scandataxp::R](R) reader structure"]
impl crate::Readable for SCANDATAXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANDATAXP to value 0"]
impl crate::Resettable for SCANDATAXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
