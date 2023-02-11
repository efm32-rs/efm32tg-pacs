#[doc = "Register `PH_DIN` reader"]
pub struct R(crate::R<PH_DIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PH_DIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PH_DIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PH_DIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN` reader - Data in"]
pub type DIN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_din](index.html) module"]
pub struct PH_DIN_SPEC;
impl crate::RegisterSpec for PH_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ph_din::R](R) reader structure"]
impl crate::Readable for PH_DIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PH_DIN to value 0"]
impl crate::Resettable for PH_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
