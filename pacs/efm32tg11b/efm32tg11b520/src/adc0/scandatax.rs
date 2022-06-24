#[doc = "Register `SCANDATAX` reader"]
pub struct R(crate::R<SCANDATAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANDATAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANDATAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANDATAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Scan Conversion Result Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCANINPUTID` reader - Scan Conversion Input ID"]
pub type SCANINPUTID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Input ID"]
    #[inline(always)]
    pub fn scaninputid(&self) -> SCANINPUTID_R {
        SCANINPUTID_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Scan Sequence Result Data + Data Source Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandatax](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct SCANDATAX_SPEC;
impl crate::RegisterSpec for SCANDATAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scandatax::R](R) reader structure"]
impl crate::Readable for SCANDATAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANDATAX to value 0"]
impl crate::Resettable for SCANDATAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
