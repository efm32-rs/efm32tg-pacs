#[doc = "Register `PG_DOUTTGL` writer"]
pub struct W(crate::W<PG_DOUTTGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_DOUTTGL_SPEC>;
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
impl From<crate::W<PG_DOUTTGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_DOUTTGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DOUTTGL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PG_DOUTTGL_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl(&mut self) -> DOUTTGL_W<0> {
        DOUTTGL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_douttgl](index.html) module"]
pub struct PG_DOUTTGL_SPEC;
impl crate::RegisterSpec for PG_DOUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pg_douttgl::W](W) writer structure"]
impl crate::Writable for PG_DOUTTGL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PG_DOUTTGL to value 0"]
impl crate::Resettable for PG_DOUTTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
