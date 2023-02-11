#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type CC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type CC1_R = crate::BitReader<bool>;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type CC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type CC2_R = crate::BitReader<bool>;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type CC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OSCFAIL` reader - OSCFAIL Interrupt Enable"]
pub type OSCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `OSCFAIL` writer - OSCFAIL Interrupt Enable"]
pub type OSCFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CNTTICK` reader - CNTTICK Interrupt Enable"]
pub type CNTTICK_R = crate::BitReader<bool>;
#[doc = "Field `CNTTICK` writer - CNTTICK Interrupt Enable"]
pub type CNTTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `MINTICK` reader - MINTICK Interrupt Enable"]
pub type MINTICK_R = crate::BitReader<bool>;
#[doc = "Field `MINTICK` writer - MINTICK Interrupt Enable"]
pub type MINTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `HOURTICK` reader - HOURTICK Interrupt Enable"]
pub type HOURTICK_R = crate::BitReader<bool>;
#[doc = "Field `HOURTICK` writer - HOURTICK Interrupt Enable"]
pub type HOURTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DAYTICK` reader - DAYTICK Interrupt Enable"]
pub type DAYTICK_R = crate::BitReader<bool>;
#[doc = "Field `DAYTICK` writer - DAYTICK Interrupt Enable"]
pub type DAYTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DAYOWOF` reader - DAYOWOF Interrupt Enable"]
pub type DAYOWOF_R = crate::BitReader<bool>;
#[doc = "Field `DAYOWOF` writer - DAYOWOF Interrupt Enable"]
pub type DAYOWOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `MONTHTICK` reader - MONTHTICK Interrupt Enable"]
pub type MONTHTICK_R = crate::BitReader<bool>;
#[doc = "Field `MONTHTICK` writer - MONTHTICK Interrupt Enable"]
pub type MONTHTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    pub fn oscfail(&self) -> OSCFAIL_R {
        OSCFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    pub fn mintick(&self) -> MINTICK_R {
        MINTICK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    pub fn hourtick(&self) -> HOURTICK_R {
        HOURTICK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    pub fn daytick(&self) -> DAYTICK_R {
        DAYTICK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    pub fn dayowof(&self) -> DAYOWOF_R {
        DAYOWOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    pub fn monthtick(&self) -> MONTHTICK_R {
        MONTHTICK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<0> {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<1> {
        CC0_W::new(self)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<2> {
        CC1_W::new(self)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<3> {
        CC2_W::new(self)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oscfail(&mut self) -> OSCFAIL_W<4> {
        OSCFAIL_W::new(self)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CNTTICK_W<5> {
        CNTTICK_W::new(self)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mintick(&mut self) -> MINTICK_W<6> {
        MINTICK_W::new(self)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hourtick(&mut self) -> HOURTICK_W<7> {
        HOURTICK_W::new(self)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn daytick(&mut self) -> DAYTICK_W<8> {
        DAYTICK_W::new(self)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dayowof(&mut self) -> DAYOWOF_W<9> {
        DAYOWOF_W::new(self)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn monthtick(&mut self) -> MONTHTICK_W<10> {
        MONTHTICK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
