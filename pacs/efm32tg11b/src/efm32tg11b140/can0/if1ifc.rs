#[doc = "Register `IF1IFC` writer"]
pub struct W(crate::W<IF1IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF1IFC_SPEC>;
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
impl From<crate::W<IF1IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF1IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` writer - Clear STATUS Interrupt Flag"]
pub type STATUS_W<'a> = crate::BitWriter<'a, u32, IF1IFC_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - Clear STATUS Interrupt Flag"]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1ifc](index.html) module"]
pub struct IF1IFC_SPEC;
impl crate::RegisterSpec for IF1IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [if1ifc::W](W) writer structure"]
impl crate::Writable for IF1IFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF1IFC to value 0"]
impl crate::Resettable for IF1IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
