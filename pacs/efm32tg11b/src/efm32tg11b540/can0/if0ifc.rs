#[doc = "Register `IF0IFC` writer"]
pub struct W(crate::W<IF0IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF0IFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IF0IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF0IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MESSAGE` writer - Clear MESSAGE Interrupt Flag"]
pub type MESSAGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IF0IFC_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Clear MESSAGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn message(&mut self) -> MESSAGE_W<0> {
        MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0ifc](index.html) module"]
pub struct IF0IFC_SPEC;
impl crate::RegisterSpec for IF0IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [if0ifc::W](W) writer structure"]
impl crate::Writable for IF0IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF0IFC to value 0"]
impl crate::Resettable for IF0IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
