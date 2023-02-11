#[doc = "Register `IF1IF` reader"]
pub struct R(crate::R<IF1IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF1IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF1IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF1IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - Status Interrupt Flag"]
pub type STATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Status Interrupt Flag"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1if](index.html) module"]
pub struct IF1IF_SPEC;
impl crate::RegisterSpec for IF1IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if1if::R](R) reader structure"]
impl crate::Readable for IF1IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF1IF to value 0"]
impl crate::Resettable for IF1IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
