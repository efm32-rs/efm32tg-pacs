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
#[doc = "Field `CMP` writer - Set CMP Interrupt Flag"]
pub type CMP_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `CONV` writer - Set CONV Interrupt Flag"]
pub type CONV_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `EOS` writer - Set EOS Interrupt Flag"]
pub type EOS_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `DMAOF` writer - Set DMAOF Interrupt Flag"]
pub type DMAOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
impl W {
    #[doc = "Bit 0 - Set CMP Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W::new(self)
    }
    #[doc = "Bit 1 - Set CONV Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&mut self) -> CONV_W {
        CONV_W::new(self)
    }
    #[doc = "Bit 2 - Set EOS Interrupt Flag"]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W::new(self)
    }
    #[doc = "Bit 3 - Set DMAOF Interrupt Flag"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DMAOF_W {
        DMAOF_W::new(self)
    }
    #[doc = "Bit 4 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W {
        APORTCONFLICT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
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
