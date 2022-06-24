#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT` writer - Set EXT Interrupt Flag"]
pub type EXT_W<'a> = crate::FieldWriter<'a, u32, IFS_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EM4WU` writer - Set EM4WU Interrupt Flag"]
pub type EM4WU_W<'a> = crate::FieldWriter<'a, u32, IFS_SPEC, u16, u16, 16, 16>;
impl W {
    #[doc = "Bits 0:15 - Set EXT Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W::new(self)
    }
    #[doc = "Bits 16:31 - Set EM4WU Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> EM4WU_W {
        EM4WU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
