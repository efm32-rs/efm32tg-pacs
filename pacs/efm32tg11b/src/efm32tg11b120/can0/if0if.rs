#[doc = "Register `IF0IF` reader"]
pub struct R(crate::R<IF0IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF0IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF0IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF0IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MESSAGE` reader - Message Object Interrupt Flag"]
pub type MESSAGE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Object Interrupt Flag"]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
#[doc = "Message Object Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0if](index.html) module"]
pub struct IF0IF_SPEC;
impl crate::RegisterSpec for IF0IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if0if::R](R) reader structure"]
impl crate::Readable for IF0IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF0IF to value 0"]
impl crate::Resettable for IF0IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
