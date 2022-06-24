#[doc = "Register `FIFODEPTH` reader"]
pub struct R(crate::R<FIFODEPTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFODEPTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFODEPTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFODEPTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - FIFO Depth."]
pub type VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Depth."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "FIFO Depth Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifodepth](index.html) module"]
pub struct FIFODEPTH_SPEC;
impl crate::RegisterSpec for FIFODEPTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifodepth::R](R) reader structure"]
impl crate::Readable for FIFODEPTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFODEPTH to value 0x40"]
impl crate::Resettable for FIFODEPTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
