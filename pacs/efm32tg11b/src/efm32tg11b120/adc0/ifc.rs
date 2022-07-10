#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLEOF` writer - Clear SINGLEOF Interrupt Flag"]
pub type SINGLEOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 8>;
#[doc = "Field `SCANOF` writer - Clear SCANOF Interrupt Flag"]
pub type SCANOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 9>;
#[doc = "Field `SINGLEUF` writer - Clear SINGLEUF Interrupt Flag"]
pub type SINGLEUF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 10>;
#[doc = "Field `SCANUF` writer - Clear SCANUF Interrupt Flag"]
pub type SCANUF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 11>;
#[doc = "Field `SINGLECMP` writer - Clear SINGLECMP Interrupt Flag"]
pub type SINGLECMP_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 16>;
#[doc = "Field `SCANCMP` writer - Clear SCANCMP Interrupt Flag"]
pub type SCANCMP_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 17>;
#[doc = "Field `VREFOV` writer - Clear VREFOV Interrupt Flag"]
pub type VREFOV_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 24>;
#[doc = "Field `PROGERR` writer - Clear PROGERR Interrupt Flag"]
pub type PROGERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 25>;
#[doc = "Field `SCANEXTPEND` writer - Clear SCANEXTPEND Interrupt Flag"]
pub type SCANEXTPEND_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 26>;
#[doc = "Field `SCANPEND` writer - Clear SCANPEND Interrupt Flag"]
pub type SCANPEND_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 27>;
#[doc = "Field `PRSTIMEDERR` writer - Clear PRSTIMEDERR Interrupt Flag"]
pub type PRSTIMEDERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 28>;
#[doc = "Field `EM23ERR` writer - Clear EM23ERR Interrupt Flag"]
pub type EM23ERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 29>;
impl W {
    #[doc = "Bit 8 - Clear SINGLEOF Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SINGLEOF_W {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - Clear SCANOF Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&mut self) -> SCANOF_W {
        SCANOF_W::new(self)
    }
    #[doc = "Bit 10 - Clear SINGLEUF Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SINGLEUF_W {
        SINGLEUF_W::new(self)
    }
    #[doc = "Bit 11 - Clear SCANUF Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> SCANUF_W {
        SCANUF_W::new(self)
    }
    #[doc = "Bit 16 - Clear SINGLECMP Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SINGLECMP_W {
        SINGLECMP_W::new(self)
    }
    #[doc = "Bit 17 - Clear SCANCMP Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> SCANCMP_W {
        SCANCMP_W::new(self)
    }
    #[doc = "Bit 24 - Clear VREFOV Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VREFOV_W {
        VREFOV_W::new(self)
    }
    #[doc = "Bit 25 - Clear PROGERR Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W::new(self)
    }
    #[doc = "Bit 26 - Clear SCANEXTPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W {
        SCANEXTPEND_W::new(self)
    }
    #[doc = "Bit 27 - Clear SCANPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> SCANPEND_W {
        SCANPEND_W::new(self)
    }
    #[doc = "Bit 28 - Clear PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W {
        PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 29 - Clear EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
