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
#[doc = "Field `UF` writer - Set UF Interrupt Flag"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `DIRCNG` writer - Set DIRCNG Interrupt Flag"]
pub type DIRCNG_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `AUXOF` writer - Set AUXOF Interrupt Flag"]
pub type AUXOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `TCC` writer - Set TCC Interrupt Flag"]
pub type TCC_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `OQSTERR` writer - Set OQSTERR Interrupt Flag"]
pub type OQSTERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - Set UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
    }
    #[doc = "Bit 1 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 2 - Set DIRCNG Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DIRCNG_W {
        DIRCNG_W::new(self)
    }
    #[doc = "Bit 3 - Set AUXOF Interrupt Flag"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AUXOF_W {
        AUXOF_W::new(self)
    }
    #[doc = "Bit 4 - Set TCC Interrupt Flag"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W::new(self)
    }
    #[doc = "Bit 5 - Set OQSTERR Interrupt Flag"]
    #[inline(always)]
    pub fn oqsterr(&mut self) -> OQSTERR_W {
        OQSTERR_W::new(self)
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
