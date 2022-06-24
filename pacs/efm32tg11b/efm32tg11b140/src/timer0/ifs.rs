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
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `UF` writer - Set UF Interrupt Flag"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `DIRCHG` writer - Set DIRCHG Interrupt Flag"]
pub type DIRCHG_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `CC0` writer - Set CC0 Interrupt Flag"]
pub type CC0_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `CC1` writer - Set CC1 Interrupt Flag"]
pub type CC1_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `CC2` writer - Set CC2 Interrupt Flag"]
pub type CC2_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `CC3` writer - Set CC3 Interrupt Flag"]
pub type CC3_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 7>;
#[doc = "Field `ICBOF0` writer - Set ICBOF0 Interrupt Flag"]
pub type ICBOF0_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 8>;
#[doc = "Field `ICBOF1` writer - Set ICBOF1 Interrupt Flag"]
pub type ICBOF1_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 9>;
#[doc = "Field `ICBOF2` writer - Set ICBOF2 Interrupt Flag"]
pub type ICBOF2_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 10>;
#[doc = "Field `ICBOF3` writer - Set ICBOF3 Interrupt Flag"]
pub type ICBOF3_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - Set UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
    }
    #[doc = "Bit 2 - Set DIRCHG Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DIRCHG_W {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 4 - Set CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W::new(self)
    }
    #[doc = "Bit 5 - Set CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W::new(self)
    }
    #[doc = "Bit 6 - Set CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W::new(self)
    }
    #[doc = "Bit 7 - Set CC3 Interrupt Flag"]
    #[inline(always)]
    pub fn cc3(&mut self) -> CC3_W {
        CC3_W::new(self)
    }
    #[doc = "Bit 8 - Set ICBOF0 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> ICBOF0_W {
        ICBOF0_W::new(self)
    }
    #[doc = "Bit 9 - Set ICBOF1 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> ICBOF1_W {
        ICBOF1_W::new(self)
    }
    #[doc = "Bit 10 - Set ICBOF2 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> ICBOF2_W {
        ICBOF2_W::new(self)
    }
    #[doc = "Bit 11 - Set ICBOF3 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof3(&mut self) -> ICBOF3_W {
        ICBOF3_W::new(self)
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
