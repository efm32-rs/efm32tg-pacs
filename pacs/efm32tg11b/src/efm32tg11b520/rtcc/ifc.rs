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
#[doc = "Field `OF` writer - Clear OF Interrupt Flag"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `CC0` writer - Clear CC0 Interrupt Flag"]
pub type CC0_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `CC1` writer - Clear CC1 Interrupt Flag"]
pub type CC1_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 2>;
#[doc = "Field `CC2` writer - Clear CC2 Interrupt Flag"]
pub type CC2_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
#[doc = "Field `OSCFAIL` writer - Clear OSCFAIL Interrupt Flag"]
pub type OSCFAIL_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
#[doc = "Field `CNTTICK` writer - Clear CNTTICK Interrupt Flag"]
pub type CNTTICK_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 5>;
#[doc = "Field `MINTICK` writer - Clear MINTICK Interrupt Flag"]
pub type MINTICK_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 6>;
#[doc = "Field `HOURTICK` writer - Clear HOURTICK Interrupt Flag"]
pub type HOURTICK_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 7>;
#[doc = "Field `DAYTICK` writer - Clear DAYTICK Interrupt Flag"]
pub type DAYTICK_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 8>;
#[doc = "Field `DAYOWOF` writer - Clear DAYOWOF Interrupt Flag"]
pub type DAYOWOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 9>;
#[doc = "Field `MONTHTICK` writer - Clear MONTHTICK Interrupt Flag"]
pub type MONTHTICK_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 10>;
impl W {
    #[doc = "Bit 0 - Clear OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - Clear CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W::new(self)
    }
    #[doc = "Bit 2 - Clear CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W::new(self)
    }
    #[doc = "Bit 3 - Clear CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W::new(self)
    }
    #[doc = "Bit 4 - Clear OSCFAIL Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&mut self) -> OSCFAIL_W {
        OSCFAIL_W::new(self)
    }
    #[doc = "Bit 5 - Clear CNTTICK Interrupt Flag"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CNTTICK_W {
        CNTTICK_W::new(self)
    }
    #[doc = "Bit 6 - Clear MINTICK Interrupt Flag"]
    #[inline(always)]
    pub fn mintick(&mut self) -> MINTICK_W {
        MINTICK_W::new(self)
    }
    #[doc = "Bit 7 - Clear HOURTICK Interrupt Flag"]
    #[inline(always)]
    pub fn hourtick(&mut self) -> HOURTICK_W {
        HOURTICK_W::new(self)
    }
    #[doc = "Bit 8 - Clear DAYTICK Interrupt Flag"]
    #[inline(always)]
    pub fn daytick(&mut self) -> DAYTICK_W {
        DAYTICK_W::new(self)
    }
    #[doc = "Bit 9 - Clear DAYOWOF Interrupt Flag"]
    #[inline(always)]
    pub fn dayowof(&mut self) -> DAYOWOF_W {
        DAYOWOF_W::new(self)
    }
    #[doc = "Bit 10 - Clear MONTHTICK Interrupt Flag"]
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
