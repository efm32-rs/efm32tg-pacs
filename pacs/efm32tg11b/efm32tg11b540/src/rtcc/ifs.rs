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
#[doc = "Field `CC0` writer - Set CC0 Interrupt Flag"]
pub type CC0_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `CC1` writer - Set CC1 Interrupt Flag"]
pub type CC1_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `CC2` writer - Set CC2 Interrupt Flag"]
pub type CC2_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `OSCFAIL` writer - Set OSCFAIL Interrupt Flag"]
pub type OSCFAIL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `CNTTICK` writer - Set CNTTICK Interrupt Flag"]
pub type CNTTICK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `MINTICK` writer - Set MINTICK Interrupt Flag"]
pub type MINTICK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `HOURTICK` writer - Set HOURTICK Interrupt Flag"]
pub type HOURTICK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 7>;
#[doc = "Field `DAYTICK` writer - Set DAYTICK Interrupt Flag"]
pub type DAYTICK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 8>;
#[doc = "Field `DAYOWOF` writer - Set DAYOWOF Interrupt Flag"]
pub type DAYOWOF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 9>;
#[doc = "Field `MONTHTICK` writer - Set MONTHTICK Interrupt Flag"]
pub type MONTHTICK_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 10>;
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - Set CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W::new(self)
    }
    #[doc = "Bit 2 - Set CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W::new(self)
    }
    #[doc = "Bit 3 - Set CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W::new(self)
    }
    #[doc = "Bit 4 - Set OSCFAIL Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&mut self) -> OSCFAIL_W {
        OSCFAIL_W::new(self)
    }
    #[doc = "Bit 5 - Set CNTTICK Interrupt Flag"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CNTTICK_W {
        CNTTICK_W::new(self)
    }
    #[doc = "Bit 6 - Set MINTICK Interrupt Flag"]
    #[inline(always)]
    pub fn mintick(&mut self) -> MINTICK_W {
        MINTICK_W::new(self)
    }
    #[doc = "Bit 7 - Set HOURTICK Interrupt Flag"]
    #[inline(always)]
    pub fn hourtick(&mut self) -> HOURTICK_W {
        HOURTICK_W::new(self)
    }
    #[doc = "Bit 8 - Set DAYTICK Interrupt Flag"]
    #[inline(always)]
    pub fn daytick(&mut self) -> DAYTICK_W {
        DAYTICK_W::new(self)
    }
    #[doc = "Bit 9 - Set DAYOWOF Interrupt Flag"]
    #[inline(always)]
    pub fn dayowof(&mut self) -> DAYOWOF_W {
        DAYOWOF_W::new(self)
    }
    #[doc = "Bit 10 - Set MONTHTICK Interrupt Flag"]
    #[inline(always)]
    pub fn monthtick(&mut self) -> MONTHTICK_W {
        MONTHTICK_W::new(self)
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
