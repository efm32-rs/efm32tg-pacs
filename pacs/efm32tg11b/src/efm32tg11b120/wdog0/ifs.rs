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
#[doc = "Field `TOUT` writer - Set TOUT Interrupt Flag"]
pub type TOUT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `WARN` writer - Set WARN Interrupt Flag"]
pub type WARN_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `WIN` writer - Set WIN Interrupt Flag"]
pub type WIN_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `PEM0` writer - Set PEM0 Interrupt Flag"]
pub type PEM0_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `PEM1` writer - Set PEM1 Interrupt Flag"]
pub type PEM1_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
impl W {
    #[doc = "Bit 0 - Set TOUT Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W::new(self)
    }
    #[doc = "Bit 1 - Set WARN Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&mut self) -> WARN_W {
        WARN_W::new(self)
    }
    #[doc = "Bit 2 - Set WIN Interrupt Flag"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W::new(self)
    }
    #[doc = "Bit 3 - Set PEM0 Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&mut self) -> PEM0_W {
        PEM0_W::new(self)
    }
    #[doc = "Bit 4 - Set PEM1 Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&mut self) -> PEM1_W {
        PEM1_W::new(self)
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
