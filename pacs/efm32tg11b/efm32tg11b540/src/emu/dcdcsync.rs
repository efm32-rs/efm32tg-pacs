#[doc = "Register `DCDCSYNC` reader"]
pub struct R(crate::R<DCDCSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCDCCTRLBUSY` reader - DCDC CTRL Register Transfer Busy"]
pub type DCDCCTRLBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DCDC CTRL Register Transfer Busy"]
    #[inline(always)]
    pub fn dcdcctrlbusy(&self) -> DCDCCTRLBUSY_R {
        DCDCCTRLBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DCDC Read Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcsync](index.html) module"]
pub struct DCDCSYNC_SPEC;
impl crate::RegisterSpec for DCDCSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcsync::R](R) reader structure"]
impl crate::Readable for DCDCSYNC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCDCSYNC to value 0"]
impl crate::Resettable for DCDCSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
